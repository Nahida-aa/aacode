//! aacode CLI：多提供商设置 + 带工具的 agent 循环。
//!
//! 设置与凭证属于应用层（见 aa-ai `docs/design/library-api.md`）：这里读
//! `settings.jsonc`、解析密钥，再把解好的端点交给库层。
//!
//! ```bash
//! aacode-cli providers                 # 列设置里的端点（不读密钥）
//! aacode-cli models minekuai           # GET /models（需要密钥）
//! aacode-cli chat minekuai deepseek "你好"        # 纯对话，不带工具
//! aacode-cli chat minekuai deepseek              # 交互模式（多轮，历史在内存）
//! aacode-cli chat minekuai deepseek --session demo  # 会话落盘，可恢复
//! aacode-cli run minekuai deepseek "把 src 里的 TODO 汇总"  # agent + 工具（只读）
//! aacode-cli run --allow-write ...     # 允许 write/edit
//! ```

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use futures::StreamExt;

use aa_agent::Session;
use aa_agent::agent_loop::AgentLoop;
use aa_agent::session::{Header, Turn};
use aa_agent::tool::ToolEvents;
use aa_language_model::provider::{
    CompletionEvent, LanguageModelClient, Model, StopReason, TokenUsage,
};
use aa_language_model::request::Request;
use aacode_settings::{Settings, default_path, derived_env_name};

mod session_file;
mod tools;

const USAGE: &str = "\
aacode — 多提供商 LLM 客户端 / coding agent

用法:
  aacode-cli [--settings <PATH>] providers
  aacode-cli [--settings <PATH>] models <provider> [--free]
  aacode-cli [--settings <PATH>] chat <provider> <model> [prompt...]
  aacode-cli [--settings <PATH>] run [选项] <provider> <model> <prompt...>
  aacode-cli run --fake [选项] <prompt...>     # 离线冒烟：不连 LLM，用剧本驱动

chat 不给 prompt 就进入交互模式（多轮，/exit 退出，/help 看命令）。

run 的选项:
  --dir <PATH>       工作目录（工具只能访问它，默认当前目录）
  --allow-write      注册 write / edit（默认只读：只有 read / glob）
  --system <TEXT>    覆盖内置系统提示（只对新建会话生效）
  --fake             用离线 fake 客户端（不读密钥、不连网络），验证 loop + 工具接线

chat / run 的会话选项:
  --session <NAME>   会话名；保存为 <sessions>/<NAME>.jsonl（只允许字母数字 ._-）
  --continue         恢复会话：配合 --session 指定，不带则取最近一次
  <sessions> 默认 {config_dir}/aacode/sessions，可用 AACODE_SESSIONS 覆盖

models 的选项:
  --free             只列免费模型（按目录里的 pricing 判断；OpenRouter 支持）

设置:
  默认 $AACODE_SETTINGS，否则 {config_dir}/aacode/settings.jsonc
  密钥: auth.env 列出的环境变量（省略则 <PROVIDER_ID>_API_KEY）
  也可以把密钥写进 `.env`（当前目录或设置文件旁；不覆盖已存在的环境变量）
";

/// 内置系统提示。工具名和规则都在这里，改工具集时同步改。
const DEFAULT_SYSTEM: &str = "\
你是一个在终端里工作的 coding agent。

规则：
- 先看再改：修改文件前用 read 看过相关代码；
- 找文件用 glob（只列路径），不要用 read 猜路径；
- 改已有文件用 edit（old_string 必须精确匹配且唯一），新建文件或整体重写用 write；
- 所有路径相对工作目录；不要访问工作目录之外的路径；
- 回答简洁，中文；不要复述文件内容，除非用户要求。";

struct Args {
    settings: PathBuf,
    dir: PathBuf,
    allow_write: bool,
    system: Option<String>,
    fake: bool,
    /// `--session`：会话名（落盘时用）。
    session: Option<String>,
    /// `--continue`：恢复已有会话。
    resume: bool,
    command: Command,
}

enum Command {
    /// 列出设置里的端点和模型（不需要密钥）。
    Providers,
    /// 调端点的 `GET /models`（需要密钥）。
    Models { provider: String, free: bool },
    /// 跑流式对话（不带工具）。`prompt` 为空时进入交互模式。
    Chat {
        provider: String,
        model: String,
        prompt: Option<String>,
    },
    /// 跑一次带工具的 agent 任务。
    Run {
        /// `--fake` 时不需要 provider / model。
        provider: Option<String>,
        model: Option<String>,
        prompt: String,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = match parse_args(std::env::args().skip(1).collect()) {
        Ok(Some(args)) => args,
        Ok(None) => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    // `--fake` 完全不碰磁盘设置：没有 settings.jsonc 也能跑冒烟。
    if args.fake && !matches!(args.command, Command::Run { .. }) {
        return Err("--fake 只能和 run 一起用".to_owned());
    }

    let settings = if args.fake {
        None
    } else {
        // 密钥可以不 export：`cargo run` 时的当前目录或设置文件旁的 `.env` 都会读。
        load_dotenv(&args.settings);

        // `providers` 只解析设置文本，不触发凭证解析；其余命令才需要密钥。
        let settings = Settings::load(&args.settings).map_err(|error| error.to_string())?;

        let unsupported = settings.unsupported_sections().collect::<Vec<_>>();
        if !unsupported.is_empty() {
            eprintln!(
                "# 注意：这些 section 还没有适配器，已忽略：{}",
                unsupported.join("、")
            );
            eprintln!("# 要连兼容 OpenAI 协议的端点，请写 openai_compatible.<id>");
        }
        Some(settings)
    };

    let settings_ref = || settings.as_ref().expect("非 fake 路径一定加载了设置");

    match args.command {
        Command::Providers => {
            list_providers(settings_ref());
            Ok(())
        }
        Command::Models { provider, free } => list_models(settings_ref(), &provider, free).await,
        Command::Chat {
            provider,
            model,
            prompt,
        } => {
            chat(
                settings_ref(),
                &provider,
                &model,
                prompt,
                args.session,
                args.resume,
            )
            .await
        }
        Command::Run {
            provider,
            model,
            prompt,
        } => {
            let options = RunOptions {
                dir: args.dir,
                allow_write: args.allow_write,
                system: args.system.unwrap_or_else(|| DEFAULT_SYSTEM.to_owned()),
                fake: args.fake,
                session: args.session,
                resume: args.resume,
            };
            agent(
                settings.as_ref(),
                provider.as_deref(),
                model.as_deref(),
                prompt,
                options,
            )
            .await
        }
    }
}

/// 读 `.env`：应用层的便利，库层不碰环境变量。
///
/// 顺序：当前目录 → 设置文件所在目录。已经存在的真实环境变量优先
/// （dotenvy 不覆盖已有值），所以 `AGNES_API_KEY=... cargo run` 仍然生效。
/// 没有 `.env` 就静默跳过 —— 凭证缺失的报错留给 `resolve`。
fn load_dotenv(settings_path: &Path) {
    let _ = dotenvy::from_filename(".env");
    if let Some(dir) = settings_path
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
    {
        let _ = dotenvy::from_path(dir.join(".env"));
    }
}

fn list_providers(settings: &Settings) {
    if let Some(proxy) = &settings.proxy {
        println!("# proxy: {proxy}");
    }

    let language_model_providers = &settings.language_model_providers;

    if language_model_providers.openai_compatible.is_empty()
        && language_model_providers.llama_cpp.is_none()
        && language_model_providers.ollama.is_none()
        && language_model_providers.openrouter.is_none()
    {
        println!("设置里没有 provider：{}", default_path().display());
        return;
    }

    for (id, provider) in &language_model_providers.openai_compatible {
        print_provider(
            id,
            &provider.base_url,
            &provider.auth,
            &provider.available_models,
            None,
        );
    }

    if let Some(provider) = &language_model_providers.llama_cpp {
        let note = if language_model_providers
            .openai_compatible
            .contains_key(aacode_settings::LLAMA_CPP_PROVIDER_ID)
        {
            "被同名 openai_compatible 条目覆盖"
        } else {
            "本地 llama.cpp（OpenAI 兼容）"
        };
        print_provider(
            aacode_settings::LLAMA_CPP_PROVIDER_ID,
            provider.base_url_or(aacode_settings::DEFAULT_LLAMA_CPP_BASE_URL),
            &provider.auth,
            &provider.available_models,
            Some(note),
        );
    }

    if let Some(provider) = &language_model_providers.openrouter {
        let note = if language_model_providers
            .openai_compatible
            .contains_key(aacode_settings::OPENROUTER_PROVIDER_ID)
        {
            "被同名 openai_compatible 条目覆盖"
        } else {
            "OpenRouter /chat/completions"
        };
        print_provider(
            aacode_settings::OPENROUTER_PROVIDER_ID,
            &provider.base_url,
            &provider.auth,
            &provider.available_models,
            Some(note),
        );
    }

    if let Some(provider) = &language_model_providers.ollama {
        let note = if language_model_providers
            .openai_compatible
            .contains_key(aacode_settings::OLLAMA_PROVIDER_ID)
        {
            "被同名 openai_compatible 条目覆盖"
        } else {
            "原生 /api/chat"
        };
        print_provider(
            aacode_settings::OLLAMA_PROVIDER_ID,
            &provider.base_url,
            &provider.auth,
            &provider.available_models,
            Some(note),
        );
    }
}

fn print_provider(
    id: &str,
    base_url: &str,
    auth: &aacode_settings::AuthConfig,
    models: &[aacode_settings::AvailableModel],
    note: Option<&str>,
) {
    match note {
        Some(note) => println!("{id} （{note}）"),
        None => println!("{id}"),
    }
    println!("  base_url: {base_url}");

    let auth = match auth {
        aacode_settings::AuthConfig::Bearer { env } if env.is_empty() => {
            format!("bearer {}", derived_env_name(id))
        }
        aacode_settings::AuthConfig::Bearer { env } => format!("bearer {}", env.join(" | ")),
        aacode_settings::AuthConfig::Header { name, env } if env.is_empty() => {
            format!("{name}: {}", derived_env_name(id))
        }
        aacode_settings::AuthConfig::Header { name, env } => format!("{name}: {}", env.join(" | ")),
        aacode_settings::AuthConfig::None => "无鉴权".to_owned(),
    };
    println!("  auth: {auth}");

    if models.is_empty() {
        println!("  models: （未声明，任意 id 可用默认事实）");
    } else {
        for model in models {
            match &model.display_name {
                Some(name) => println!("  model: {} ({name})", model.name),
                None => println!("  model: {}", model.name),
            }
        }
    }
}

async fn list_models(settings: &Settings, provider: &str, free: bool) -> Result<(), String> {
    let resolved = settings
        .resolve(provider)
        .map_err(|error| error.to_string())?;

    println!("# {} ({})", resolved.id, resolved.base_url);
    let models = resolved
        .client
        .list_models()
        .await
        .map_err(|error| error.to_string())?;

    // `--free` 只能筛「目录里带定价」的端点（OpenRouter）；其余端点没有这个信息，
    // 明确提示而不是列出一份空表让人以为配置错了。
    if free && models.iter().all(|model| model.free.is_none()) {
        return Err(format!(
            "`{provider}` 的模型目录不提供定价，无法判断免费；去掉 --free"
        ));
    }

    let mut shown = 0usize;
    for model in models {
        if free && model.free != Some(true) {
            continue;
        }
        shown += 1;
        let mark = if resolved.declares_model(&model.id) {
            "*"
        } else {
            " "
        };
        let price = if model.free == Some(true) {
            "[free] "
        } else {
            ""
        };
        match model.owned_by {
            Some(owner) => println!("{mark} {price}{}\t{owner}", model.id),
            None => println!("{mark} {price}{}", model.id),
        }
    }
    if shown == 0 {
        println!("# 目录里没有免费模型");
    }
    println!("# * = 设置里有模型事实；其余用默认事实也能 chat");
    Ok(())
}

async fn chat(
    settings: &Settings,
    provider: &str,
    model_id: &str,
    prompt: Option<String>,
    session_name: Option<String>,
    resume: bool,
) -> Result<(), String> {
    let resolved = settings
        .resolve(provider)
        .map_err(|error| error.to_string())?;
    let model = resolved.model(model_id);

    if !resolved.declares_model(model_id) {
        eprintln!("# 设置里没有模型 `{model_id}`，使用默认事实（不一定准确）");
    }

    let cwd = current_dir_display();
    let (mut session, mut file) = open_session(session_name, resume, provider, model_id, cwd, "")?;

    let Some(prompt) = prompt else {
        return chat_repl(&resolved, &model, &mut session, &mut file).await;
    };

    session.push_user_text(prompt);
    chat_turn(&resolved, &model, &mut session, true).await?;
    persist(&mut file, &session)
}

/// 交互模式：一行一轮，历史留在同一个会话里。
async fn chat_repl(
    resolved: &aacode_settings::ResolvedProvider,
    model: &Model,
    session: &mut Session,
    file: &mut Option<session_file::SessionFile>,
) -> Result<(), String> {
    eprintln!("# 交互模式：输入内容回车发送；/help 看命令，/exit 退出");

    let mut input = String::new();
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        input.clear();
        let read = std::io::stdin()
            .read_line(&mut input)
            .map_err(|error| format!("读输入失败：{error}"))?;
        if read == 0 {
            // Ctrl-D。
            println!();
            return Ok(());
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }
        match line {
            "/exit" | "/quit" => return Ok(()),
            "/help" => {
                println!("  /exit, /quit  退出");
                println!("  /help         这条帮助");
                println!("  其他输入      发给模型（历史会累积）");
                continue;
            }
            _ => {}
        }

        session.push_user_text(line);
        chat_turn(resolved, model, session, false).await?;
        persist(file, session)?;
    }
}

/// 跑一轮对话：把 `session` 的历史发给模型，流式打印，再把这一轮写回会话。
async fn chat_turn(
    resolved: &aacode_settings::ResolvedProvider,
    model: &Model,
    session: &mut Session,
    verbose: bool,
) -> Result<(), String> {
    let request = Request {
        messages: session.messages().to_vec(),
        stream: true,
        // 设置里的 reasoning_effort 是每请求选择的默认值，不进 Model。
        reasoning_effort: resolved.reasoning_effort(&model.id),
        ..Request::default()
    };

    let mut stream = resolved
        .client
        .stream_completion(model, &request)
        .await
        .map_err(|error| error.to_string())?;

    let mut text = String::new();
    let mut reasoning = String::new();
    let mut signature: Option<String> = None;
    let mut usage: Option<TokenUsage> = None;
    let mut stop: Option<StopReason> = None;

    while let Some(event) = stream.next().await {
        match event.map_err(|error| error.to_string())? {
            CompletionEvent::Text(delta) => {
                print!("{delta}");
                let _ = std::io::stdout().flush();
                text.push_str(&delta);
            }
            CompletionEvent::Reasoning {
                text: delta,
                signature: delta_signature,
            } => {
                // 暗色显示思考；不支持思考的端点和模型不会发这个事件。
                print!("\x1b[2m{delta}\x1b[0m");
                let _ = std::io::stdout().flush();
                reasoning.push_str(&delta);
                if delta_signature.is_some() {
                    signature = delta_signature;
                }
            }
            CompletionEvent::ToolUse { name, .. } => {
                // chat 不注册工具；模型主动要求工具时说明清楚，而不是静默丢弃。
                eprintln!("\n[tool] {name}（chat 不带工具，已忽略）");
            }
            CompletionEvent::Usage(value) => usage = Some(value),
            CompletionEvent::Stop(reason) => {
                stop = Some(reason);
                break;
            }
        }
    }

    println!();
    if verbose {
        match stop {
            Some(reason) => println!("[stop] {reason:?}"),
            None => eprintln!("[warn] 流结束了但没有 Stop 事件"),
        }
        if let Some(usage) = usage {
            println!(
                "[usage] input={} output={}",
                usage.input_tokens, usage.output_tokens
            );
        }
    } else if stop.is_none() {
        eprintln!("[warn] 流结束了但没有 Stop 事件");
    }

    session.push_assistant(Turn {
        text,
        reasoning: (!reasoning.is_empty()).then_some(reasoning),
        signature,
        tool_calls: Vec::new(),
        stop,
        usage,
    });
    Ok(())
}

/// 打开会话：新建、按名字恢复，或恢复最近一次。
///
/// - 不给 `--session` / `--continue`：纯内存会话，不落盘；
/// - `--session NAME`：新会话，文件已存在时报错（要接着写请加 `--continue`）；
/// - `--continue [--session NAME]`：从文件恢复，沿用文件里的系统提示。
fn open_session(
    name: Option<String>,
    resume: bool,
    provider: &str,
    model: &str,
    cwd: String,
    system: &str,
) -> Result<(Session, Option<session_file::SessionFile>), String> {
    let path = match (resume, &name) {
        (_, Some(name)) => Some(session_file::path_for(name)?),
        (true, None) => Some(session_file::latest()?),
        (false, None) => None,
    };
    let Some(path) = path else {
        let session = Session::new(Header::new(provider, model, cwd, system))
            .map_err(|error| error.to_string())?;
        return Ok((session, None));
    };

    if resume {
        let session = session_file::load(&path)?;
        eprintln!(
            "# 恢复会话：{}（{} 条消息）",
            path.display(),
            session.message_count()
        );
        let header = session.header();
        if header.provider != provider || header.model != model {
            eprintln!(
                "# 注意：会话建立时用的是 {} / {}，本次是 {provider} / {model}",
                header.provider, header.model
            );
        }
        if !system.is_empty() && session.system() != system {
            eprintln!("# 注意：沿用会话里的系统提示，忽略本次的 --system");
        }
        let file = session_file::SessionFile::resumed(path, &session);
        return Ok((session, Some(file)));
    }

    if path.exists() {
        return Err(format!(
            "会话 {} 已存在；用 --continue 接着写，或换一个 --session 名字",
            path.display()
        ));
    }
    let session = Session::new(Header::new(provider, model, cwd, system))
        .map_err(|error| error.to_string())?;
    eprintln!("# 新会话：{}", path.display());
    Ok((session, Some(session_file::SessionFile::new(path))))
}

/// 把新增事件写进会话文件（没有会话文件时什么也不做）。
fn persist(file: &mut Option<session_file::SessionFile>, session: &Session) -> Result<(), String> {
    if let Some(file) = file {
        file.sync(session)?;
    }
    Ok(())
}

fn current_dir_display() -> String {
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_default()
}

/// 带工具的 agent 任务。
/// `run` 的选项（参数太多了，打包成结构免得 clippy 报 8 个参数）。
struct RunOptions {
    dir: PathBuf,
    allow_write: bool,
    system: String,
    fake: bool,
    session: Option<String>,
    resume: bool,
}

async fn agent(
    settings: Option<&Settings>,
    provider: Option<&str>,
    model_id: Option<&str>,
    prompt: String,
    options: RunOptions,
) -> Result<(), String> {
    let RunOptions {
        dir,
        allow_write,
        system,
        fake,
        session: session_name,
        resume,
    } = options;
    let workspace = Arc::new(tools::Workspace::new(dir, allow_write));
    let registry = tools::registry(workspace.clone());
    let cwd = workspace.root().display().to_string();

    eprintln!("# 工作目录: {}", workspace.root().display());
    eprintln!(
        "# 工具: {}{}",
        registry.names().join(", "),
        if allow_write {
            ""
        } else {
            "（只读；加 --allow-write 启用 write/edit）"
        }
    );

    if fake {
        // 离线冒烟：剧本固定为「先 glob，再回一段话」，用来验证 loop + 工具接线，
        // 不读设置、不碰网络。真模型的行为不在这里验证。
        eprintln!("# fake 模式：不连 LLM");
        let client = aa_language_models::fake::FakeClient::with_rounds([
            vec![
                aa_language_model::provider::CompletionEvent::ToolUse {
                    id: "call_1".to_owned(),
                    name: "glob".to_owned(),
                    input: serde_json::json!({ "pattern": "**/*.rs", "limit": 3 }),
                    is_complete: true,
                },
                aa_language_model::provider::CompletionEvent::Stop(
                    aa_language_model::provider::StopReason::ToolUse,
                ),
            ],
            vec![
                aa_language_model::provider::CompletionEvent::Text(
                    "（fake）已通过 glob 看过文件列表。".to_owned(),
                ),
                aa_language_model::provider::CompletionEvent::Stop(
                    aa_language_model::provider::StopReason::EndTurn,
                ),
            ],
        ]);
        let model = aa_language_model::provider::Model::new("fake", "fake");
        // fake 也走会话：离线验证「落盘 → 恢复 → 继续」的完整链路。
        let (mut session, mut file) =
            open_session(session_name, resume, "fake", "fake", cwd, &system)?;
        session.push_user_text(&prompt);
        AgentLoop::new(Arc::new(client), registry)
            .run(&model, &mut session, &CliEvents)
            .await
            .map_err(|error| error.to_string())?;
        persist(&mut file, &session)?;
        println!();
        return Ok(());
    }

    let provider = provider.ok_or("run 需要 <provider> <model> <prompt>")?;
    let model_id = model_id.ok_or("run 需要 <provider> <model> <prompt>")?;
    let resolved = settings
        .expect("非 fake 路径一定加载了设置")
        .resolve(provider)
        .map_err(|error| error.to_string())?;
    let model = resolved.model(model_id);

    if !resolved.declares_model(model_id) {
        eprintln!("# 设置里没有模型 `{model_id}`，使用默认事实（不一定准确）");
    }

    let (mut session, mut file) =
        open_session(session_name, resume, provider, model_id, cwd, &system)?;
    session.push_user_text(&prompt);

    AgentLoop::new(Arc::new(resolved.client), registry)
        .run(&model, &mut session, &CliEvents)
        .await
        .map_err(|error| error.to_string())?;
    persist(&mut file, &session)?;

    // 文本已经在事件回调里流式打印过，这里只收尾。
    println!();
    Ok(())
}

/// 把 agent 的进度回调转成终端输出。
///
/// 库层只有一个「进度字符串」通道：文本增量原样打印；工具/用量这类结构化信息
/// 带前缀，用暗色单独起一行。
struct CliEvents;

impl ToolEvents for CliEvents {
    fn progress(&self, message: &str) {
        if let Some(rest) = message.strip_prefix("[usage] ") {
            eprintln!("\n\x1b[2m[usage] {rest}\x1b[0m");
        } else if message.starts_with("→ tool: ") {
            // 工具调用单独起一行、暗色，避免和模型正文粘在一起。
            eprintln!("\n\x1b[2m{message}\x1b[0m");
        } else {
            print!("{message}");
        }
        let _ = std::io::stdout().flush();
    }

    fn cancelled(&self) -> bool {
        false
    }
}

/// `Ok(None)` = 打印用法并正常退出。
///
/// 选项可以写在命令前也可以写在命令后（`--fake run ...` 与 `run --fake ...` 等价）：
/// 扫到一个非选项 token 且还没有命令时，它就是命令名；之后的非选项都是位置参数。
fn parse_args(raw: Vec<String>) -> Result<Option<Args>, String> {
    let mut settings = None;
    let mut dir = None;
    let mut allow_write = false;
    let mut system = None;
    let mut fake = false;
    let mut free = false;
    let mut session = None;
    let mut resume = false;
    let mut command_name: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();

    let mut iter = raw.into_iter();
    while let Some(argument) = iter.next() {
        match argument.as_str() {
            "-h" | "--help" | "help" if command_name.is_none() => return Ok(None),
            // `--config` 是旧名，保留但提示；设置文件一直叫 settings.jsonc。
            "--settings" | "--config" => {
                let value = iter
                    .next()
                    .ok_or_else(|| format!("{argument} 需要一个路径"))?;
                if argument == "--config" {
                    eprintln!("# --config 已改名 --settings");
                }
                settings = Some(PathBuf::from(value));
            }
            "--dir" => {
                let value = iter.next().ok_or("--dir 需要一个路径")?;
                dir = Some(PathBuf::from(value));
            }
            "--system" => {
                let value = iter.next().ok_or("--system 需要一段文本")?;
                system = Some(value);
            }
            "--allow-write" => allow_write = true,
            "--fake" => fake = true,
            "--free" => free = true,
            "--session" => {
                let value = iter.next().ok_or("--session 需要一个名字")?;
                session = Some(value);
            }
            "--continue" => resume = true,
            _ if argument.starts_with("--settings=") || argument.starts_with("--config=") => {
                let (flag, value) = argument.split_once('=').expect("匹配了以 = 开头的分支");
                if value.is_empty() {
                    return Err(format!("{flag}= 后面需要一个路径"));
                }
                if flag == "--config" {
                    eprintln!("# --config 已改名 --settings");
                }
                settings = Some(PathBuf::from(value));
            }
            _ if argument.starts_with("--dir=") => {
                let value = &argument["--dir=".len()..];
                if value.is_empty() {
                    return Err("--dir= 后面需要一个路径".to_owned());
                }
                dir = Some(PathBuf::from(value));
            }
            _ if argument.starts_with("--system=") => {
                system = Some(argument["--system=".len()..].to_owned());
            }
            _ if argument.starts_with("--session=") => {
                let value = &argument["--session=".len()..];
                if value.is_empty() {
                    return Err("--session= 后面需要一个名字".to_owned());
                }
                session = Some(value.to_owned());
            }
            _ if argument.starts_with('-') => {
                return Err(format!("未知选项 `{argument}`"));
            }
            _ if command_name.is_none() => command_name = Some(argument),
            _ => positional.push(argument),
        }
    }

    let Some(name) = command_name else {
        return Ok(None);
    };

    let command = match name.as_str() {
        "providers" => {
            expect_no_positional(&name, &positional)?;
            Command::Providers
        }
        "models" => {
            let provider = positional
                .first()
                .cloned()
                .ok_or("models 需要 provider id（先用 providers 看有哪些）")?;
            expect_no_positional(&name, &positional[1..])?;
            Command::Models { provider, free }
        }
        "chat" | "run" => {
            // `run --fake <prompt>`：不需要 provider / model。
            if name == "run" && fake {
                if positional.is_empty() {
                    return Err("run --fake 需要 <prompt>".to_owned());
                }
                Command::Run {
                    provider: None,
                    model: None,
                    prompt: positional.join(" "),
                }
            } else {
                let minimum = if name == "chat" { 2 } else { 3 };
                if positional.len() < minimum {
                    return Err(if name == "chat" {
                        "chat 需要 <provider> <model> [prompt...]（不给 prompt 进入交互模式）"
                            .to_owned()
                    } else {
                        format!("{name} 需要 <provider> <model> <prompt>")
                    });
                }
                let mut rest = positional.into_iter();
                let provider = rest.next().expect("len >= minimum");
                let model = rest.next().expect("len >= minimum");
                let prompt = rest.collect::<Vec<_>>().join(" ");
                if name == "chat" {
                    Command::Chat {
                        provider,
                        model,
                        prompt: (!prompt.is_empty()).then_some(prompt),
                    }
                } else {
                    Command::Run {
                        provider: Some(provider),
                        model: Some(model),
                        prompt,
                    }
                }
            }
        }
        other => return Err(format!("未知命令 `{other}`")),
    };

    if free && !matches!(command, Command::Models { .. }) {
        return Err("--free 只能和 models 一起用".to_owned());
    }
    if (session.is_some() || resume)
        && !matches!(command, Command::Chat { .. } | Command::Run { .. })
    {
        return Err("--session / --continue 只能和 chat / run 一起用".to_owned());
    }
    if system.is_some() && !matches!(command, Command::Run { .. }) {
        return Err("--system 只能和 run 一起用".to_owned());
    }

    Ok(Some(Args {
        settings: settings.unwrap_or_else(default_path),
        dir: dir.unwrap_or_else(|| PathBuf::from(".")),
        allow_write,
        system,
        fake,
        session,
        resume,
        command,
    }))
}

/// 多余的位置参数是笔误，直接报错而不是忽略。
fn expect_no_positional(command: &str, extra: &[String]) -> Result<(), String> {
    if extra.is_empty() {
        Ok(())
    } else {
        Err(format!("{command} 不接受多余参数：{extra:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(raw: &[&str]) -> Result<Option<Args>, String> {
        parse_args(raw.iter().map(|argument| argument.to_string()).collect())
    }

    /// 只关心命令形状的测试：省略路径，避免依赖真实的 config dir。
    fn parse_command(raw: &[&str]) -> Command {
        parse(raw).unwrap().unwrap().command
    }

    #[test]
    fn chat_without_prompt_enters_repl() {
        let Command::Chat { prompt, .. } = parse_command(&["chat", "agnes", "agnes-3.0-flash"])
        else {
            panic!("应该是 Chat");
        };
        assert!(prompt.is_none());

        let Command::Chat { prompt, .. } =
            parse_command(&["chat", "agnes", "agnes-3.0-flash", "你好", "呀"])
        else {
            panic!("应该是 Chat");
        };
        assert_eq!(prompt.as_deref(), Some("你好 呀"));
    }

    #[test]
    fn session_flags_are_parsed_for_chat_and_run() {
        let args = parse(&["chat", "agnes", "m", "--session", "demo", "--continue"])
            .unwrap()
            .unwrap();
        assert_eq!(args.session.as_deref(), Some("demo"));
        assert!(args.resume);

        let args = parse(&["run", "--session=demo", "agnes", "m", "go"])
            .unwrap()
            .unwrap();
        assert_eq!(args.session.as_deref(), Some("demo"));
        assert!(!args.resume);
    }

    #[test]
    fn session_and_continue_are_rejected_elsewhere() {
        assert!(parse(&["providers", "--session", "demo"]).is_err());
        assert!(parse(&["models", "agnes", "--continue"]).is_err());
        assert!(parse(&["--session=", "chat", "a", "m"]).is_err());
    }

    #[test]
    fn system_is_run_only() {
        assert!(parse(&["chat", "a", "m", "--system", "x"]).is_err());
        assert!(parse(&["run", "a", "m", "go", "--system", "x"]).is_ok());
    }
}
