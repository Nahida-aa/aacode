//! aacode CLI：先用多提供商设置把真实 LLM 跑起来。
//!
//! 设置与凭证属于应用层（见 aa-ai `docs/design/library-api.md`）：这里读
//! `settings.jsonc`、解析密钥，再把解好的端点交给库层。
//!
//! ```bash
//! aa-agent-cli providers              # 列设置里的端点（不读密钥）
//! aa-agent-cli models minekuai        # GET /models（需要密钥）
//! aa-agent-cli chat agnes agnes-2.0-flash "你好"
//! ```

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use futures::StreamExt;

use aa_language_model::provider::{CompletionEvent, LanguageModelClient, StopReason, TokenUsage};
use aa_language_model::request::Request;
use aa_language_model::request::message::{ContentPart, Message, Role};
use aa_settings::{Settings, default_path, derived_env_name};

const USAGE: &str = "\
aacode — 多提供商 LLM 客户端

用法:
  aa-agent-cli [--config <PATH>] providers
  aa-agent-cli [--config <PATH>] models <provider>
  aa-agent-cli [--config <PATH>] chat <provider> <model> [prompt...]

设置:
  默认 $AACODE_SETTINGS，否则 {config_dir}/aacode/settings.jsonc
  密钥: auth.env 列出的环境变量（省略则 <PROVIDER_ID>_API_KEY）
";

struct Args {
    settings: PathBuf,
    command: Command,
}

enum Command {
    /// 列出设置里的端点和模型（不需要密钥）。
    Providers,
    /// 调端点的 `GET /models`（需要密钥）。
    Models { provider: String },
    /// 跑一次流式对话。
    Chat {
        provider: String,
        model: String,
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
    // `providers` 只解析设置文本，不触发凭证解析；models/chat 才需要密钥。
    let settings = Settings::load(&args.settings).map_err(|error| error.to_string())?;

    let unsupported = settings.unsupported_sections().collect::<Vec<_>>();
    if !unsupported.is_empty() {
        eprintln!(
            "# 注意：这些 section 还没有适配器，已忽略：{}",
            unsupported.join("、")
        );
        eprintln!("# 要连兼容 OpenAI 协议的端点，请写 openai_compatible.<id>");
    }

    match args.command {
        Command::Providers => {
            list_providers(&settings);
            Ok(())
        }
        Command::Models { provider } => list_models(&settings, &provider).await,
        Command::Chat {
            provider,
            model,
            prompt,
        } => chat(&settings, &provider, &model, prompt).await,
    }
}

fn list_providers(settings: &Settings) {
    let language_models = &settings.language_models;

    if language_models.openai_compatible.is_empty() && language_models.ollama.is_none() {
        println!("设置里没有 provider：{}", default_path().display());
        return;
    }

    for (id, provider) in &language_models.openai_compatible {
        print_provider(
            id,
            &provider.base_url,
            &provider.auth,
            &provider.available_models,
            None,
        );
    }

    if let Some(provider) = &language_models.ollama {
        let note = if language_models
            .openai_compatible
            .contains_key(aa_settings::OLLAMA_PROVIDER_ID)
        {
            "被同名 openai_compatible 条目覆盖"
        } else {
            "过渡实现：走 OpenAI 兼容 /v1"
        };
        print_provider(
            aa_settings::OLLAMA_PROVIDER_ID,
            &provider.openai_base_url(),
            &provider.auth,
            &provider.available_models,
            Some(note),
        );
    }
}

fn print_provider(
    id: &str,
    base_url: &str,
    auth: &aa_settings::AuthConfig,
    models: &[aa_settings::AvailableModel],
    note: Option<&str>,
) {
    match note {
        Some(note) => println!("{id} （{note}）"),
        None => println!("{id}"),
    }
    println!("  base_url: {base_url}");

    let auth = match auth {
        aa_settings::AuthConfig::Bearer { env } if env.is_empty() => {
            format!("bearer {}", derived_env_name(id))
        }
        aa_settings::AuthConfig::Bearer { env } => format!("bearer {}", env.join(" | ")),
        aa_settings::AuthConfig::Header { name, env } if env.is_empty() => {
            format!("{name}: {}", derived_env_name(id))
        }
        aa_settings::AuthConfig::Header { name, env } => format!("{name}: {}", env.join(" | ")),
        aa_settings::AuthConfig::None => "无鉴权".to_owned(),
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

async fn list_models(settings: &Settings, provider: &str) -> Result<(), String> {
    let resolved = settings
        .resolve(provider)
        .map_err(|error| error.to_string())?;

    println!("# {} ({})", resolved.id, resolved.base_url);
    let models = resolved
        .client
        .list_models()
        .await
        .map_err(|error| error.to_string())?;

    for model in models {
        let mark = if resolved.declares_model(&model.id) {
            "*"
        } else {
            " "
        };
        match model.owned_by {
            Some(owner) => println!("{mark} {}\t{owner}", model.id),
            None => println!("{mark} {}", model.id),
        }
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

/// `Ok(None)` = 打印用法并正常退出。
fn parse_args(mut raw: Vec<String>) -> Result<Option<Args>, String> {
    let mut settings = None;

    while let Some(argument) = raw.first() {
        match argument.as_str() {
            "-h" | "--help" | "help" => return Ok(None),
            "--config" => {
                raw.remove(0);
                if raw.is_empty() {
                    return Err("--config 需要一个路径".to_owned());
                }
                settings = Some(PathBuf::from(raw.remove(0)));
            }
            _ if argument.starts_with("--config=") => {
                let path = argument["--config=".len()..].to_owned();
                if path.is_empty() {
                    return Err("--config= 后面需要一个路径".to_owned());
                }
                raw.remove(0);
                settings = Some(PathBuf::from(path));
            }
            _ => break,
        }
    }

    let Some(name) = raw.first().cloned() else {
        return Ok(None);
    };
    raw.remove(0);

    let command = match name.as_str() {
        "providers" => Command::Providers,
        "models" => {
            let provider = raw
                .first()
                .cloned()
                .ok_or("models 需要 provider id（先用 providers 看有哪些）")?;
            raw.remove(0);
            Command::Models { provider }
        }
        "chat" => {
            if raw.len() < 3 {
                return Err("chat 需要 <provider> <model> <prompt>".to_owned());
            }
            let provider = raw.remove(0);
            let model = raw.remove(0);
            let prompt = raw.join(" ");
            raw.clear();
            Command::Chat {
                provider,
                model,
                prompt,
            }
        }
        other => return Err(format!("未知命令 `{other}`")),
    };

    if !raw.is_empty() {
        return Err(format!("多余的参数：{raw:?}"));
    }

    Ok(Some(Args {
        settings: settings.unwrap_or_else(default_path),
        command,
    }))
}
