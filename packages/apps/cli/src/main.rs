//! aacode CLI：多提供商设置 + 带工具的 agent 循环。
//!
//! 设置与凭证属于应用层（见 aa-ai `docs/design/library-api.md`）：这里读
//! `settings.jsonc`、解析密钥，再把解好的端点交给库层。
//!
//! ```bash
//! aacode-cli providers                 # 列设置里的端点（不读密钥）
//! aacode-cli models minekuai           # GET /models（需要密钥）
//! aacode-cli chat minekuai deepseek "你好"        # 纯对话，不带工具
//! aacode-cli run minekuai deepseek "把 src 里的 TODO 汇总"  # agent + 工具（只读）
//! aacode-cli run --allow-write ...     # 允许 write/edit
//! ```

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use futures::StreamExt;

use aa_agent::AgentLoop;
use aa_agent::tool::ToolEvents;
use aa_language_model::provider::{CompletionEvent, LanguageModelClient, StopReason, TokenUsage};
use aa_language_model::request::Request;
use aa_language_model::request::message::{ContentPart, Message, Role};
use aacode_settings::{Settings, default_path, derived_env_name};

mod tools;

const USAGE: &str = "\
aacode — 多提供商 LLM 客户端 / coding agent

用法:
  aacode-cli [--settings <PATH>] providers
  aacode-cli [--settings <PATH>] models <provider> [--free]
  aacode-cli [--settings <PATH>] chat <provider> <model> [prompt...]
  aacode-cli [--settings <PATH>] run [选项] <provider> <model> <prompt...>
  aacode-cli run --stub [选项] <prompt...>     # 离线冒烟：不连 LLM，用剧本驱动

run 的选项:
  --dir <PATH>       工作目录（工具只能访问它，默认当前目录）
  --allow-write      注册 write / edit（默认只读：只有 read / glob）
  --system <TEXT>    覆盖内置系统提示
  --stub             用离线 stub 客户端（不读密钥、不连网络），验证 loop + 工具接线

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
    stub: bool,
    command: Command,
}

enum Command {
    /// 列出设置里的端点和模型（不需要密钥）。
    Providers,
    /// 调端点的 `GET /models`（需要密钥）。
    Models { provider: String, free: bool },
    /// 跑一次流式对话（不带工具）。
    Chat {
        provider: String,
        model: String,
        prompt: String,
    },
    /// 跑一次带工具的 agent 任务。
    Run {
        /// `--stub` 时不需要 provider / model。
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
    // `--stub` 完全不碰磁盘设置：没有 settings.jsonc 也能跑冒烟。
    if args.stub && !matches!(args.command, Command::Run { .. }) {
        return Err("--stub 只能和 run 一起用".to_owned());
    }

    let settings = if args.stub {
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

    let settings_ref = || settings.as_ref().expect("非 stub 路径一定加载了设置");

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
        } => chat(settings_ref(), &provider, &model, prompt).await,
        Command::Run {
            provider,
            model,
            prompt,
        } => {
            let options = RunOptions {
                dir: args.dir,
                allow_write: args.allow_write,
                system: args.system.unwrap_or_else(|| DEFAULT_SYSTEM.to_owned()),
                stub: args.stub,
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
    prompt: String,
) -> Result<(), String> {
    let resolved = settings
        .resolve(provider)
        .map_err(|error| error.to_string())?;
    let model = resolved.model(model_id);

    if !resolved.declares_model(model_id) {
        eprintln!("# 设置里没有模型 `{model_id}`，使用默认事实（不一定准确）");
    }

    let request = Request {
        messages: vec![Arc::new(Message::new(
            Role::User,
            vec![ContentPart::text(prompt)],
        ))],
        stream: true,
        // 设置里的 reasoning_effort 是每请求选择的默认值，不进 Model。
        reasoning_effort: resolved.reasoning_effort(model_id),
        ..Request::default()
    };

    let mut stream = resolved
        .client
        .stream_completion(&model, &request)
        .await
        .map_err(|error| error.to_string())?;

    let mut usage: Option<TokenUsage> = None;
    let mut stop: Option<StopReason> = None;

    while let Some(event) = stream.next().await {
        match event.map_err(|error| error.to_string())? {
            CompletionEvent::Text(text) => {
                print!("{text}");
                let _ = std::io::stdout().flush();
            }
            CompletionEvent::Reasoning { text, .. } => {
                // 暗色显示思考；不支持思考的端点和模型不会发这个事件。
                print!("\x1b[2m{text}\x1b[0m");
                let _ = std::io::stdout().flush();
            }
            CompletionEvent::ToolUse { name, .. } => {
                eprintln!("\n[tool] {name}");
            }
            CompletionEvent::Usage(value) => usage = Some(value),
            CompletionEvent::Stop(reason) => {
                stop = Some(reason);
                break;
            }
        }
    }

    println!();
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

    Ok(())
}

/// 带工具的 agent 任务。
/// `run` 的选项（参数太多了，打包成结构免得 clippy 报 8 个参数）。
struct RunOptions {
    dir: PathBuf,
    allow_write: bool,
    system: String,
    stub: bool,
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
        stub,
    } = options;
    let workspace = Arc::new(tools::Workspace::new(dir, allow_write));
    let registry = tools::registry(workspace.clone());

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

    if stub {
        // 离线冒烟：剧本固定为「先 glob，再回一段话」，用来验证 loop + 工具接线，
        // 不读设置、不碰网络。真模型的行为不在这里验证。
        eprintln!("# stub 模式：不连 LLM");
        let client = aa_language_models::stub::StubClient::with_rounds([
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
                    "（stub）已通过 glob 看过文件列表。".to_owned(),
                ),
                aa_language_model::provider::CompletionEvent::Stop(
                    aa_language_model::provider::StopReason::EndTurn,
                ),
            ],
        ]);
        let model = aa_language_model::provider::Model::new("stub", "stub");
        AgentLoop::new(Arc::new(client), registry)
            .run(&model, &system, &prompt, &CliEvents)
            .await
            .map_err(|error| error.to_string())?;
        println!();
        return Ok(());
    }

    let provider = provider.ok_or("run 需要 <provider> <model> <prompt>")?;
    let model_id = model_id.ok_or("run 需要 <provider> <model> <prompt>")?;
    let resolved = settings
        .expect("非 stub 路径一定加载了设置")
        .resolve(provider)
        .map_err(|error| error.to_string())?;
    let model = resolved.model(model_id);

    if !resolved.declares_model(model_id) {
        eprintln!("# 设置里没有模型 `{model_id}`，使用默认事实（不一定准确）");
    }

    let agent = AgentLoop::new(Arc::new(resolved.client), registry);
    agent
        .run(&model, &system, &prompt, &CliEvents)
        .await
        .map_err(|error| error.to_string())?;

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
/// 选项可以写在命令前也可以写在命令后（`--stub run ...` 与 `run --stub ...` 等价）：
/// 扫到一个非选项 token 且还没有命令时，它就是命令名；之后的非选项都是位置参数。
fn parse_args(raw: Vec<String>) -> Result<Option<Args>, String> {
    let mut settings = None;
    let mut dir = None;
    let mut allow_write = false;
    let mut system = None;
    let mut stub = false;
    let mut free = false;
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
            "--stub" => stub = true,
            "--free" => free = true,
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
            // `run --stub <prompt>`：不需要 provider / model。
            if name == "run" && stub {
                if positional.is_empty() {
                    return Err("run --stub 需要 <prompt>".to_owned());
                }
                Command::Run {
                    provider: None,
                    model: None,
                    prompt: positional.join(" "),
                }
            } else {
                if positional.len() < 3 {
                    return Err(format!("{name} 需要 <provider> <model> <prompt>"));
                }
                let mut rest = positional.into_iter();
                let provider = rest.next().expect("len >= 3");
                let model = rest.next().expect("len >= 3");
                let prompt = rest.collect::<Vec<_>>().join(" ");
                if name == "chat" {
                    Command::Chat {
                        provider,
                        model,
                        prompt,
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

    Ok(Some(Args {
        settings: settings.unwrap_or_else(default_path),
        dir: dir.unwrap_or_else(|| PathBuf::from(".")),
        allow_write,
        system,
        stub,
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
