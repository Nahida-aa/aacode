//! 端点配置形状 + 解析成库层客户端。
//!
//! 两种 section，名字代表**协议**而不是厂商（对齐 Zed）：
//! - [`OpenAiCompatibleProvider`]：`language_models.openai_compatible.<id>`，走 `/chat/completions`；
//! - [`OllamaProvider`]：`language_models.ollama`，语义上是原生 `/api/chat`，但库层
//!   还没有原生适配器，所以当前**过渡实现走 OpenAI 兼容 `/v1`**。
//!
//! 字段名对齐 Zed `OpenAiCompatibleSettingsContent` / `OpenAiCompatibleAvailableModel`
//! （`settings_content/src/language_model.rs:424`）：
//! - `api_url` / `available_models` / `headers`；
//! - 模型事实 `context_window`（Zed 叫 `max_tokens`，但那是上下文窗口，不是输出上限）
//!   与 `max_output_tokens`；
//! - `capabilities` 的默认值与 Zed 相同（`tools` 开、`images` 关、
//!   `max_tokens_parameter` 关 → 默认发 `max_completion_tokens`）。
//!
//! `auth` 是 aa-ai 的扩展：Zed 把密钥放钥匙串，这里放「变量名」。

use std::collections::BTreeMap;

use aa_ai_openai::OpenAiLanguageModelProvider;
use aa_language_model::provider::{Model, ReasoningEffort};
use serde::Deserialize;

use crate::credentials::Credentials;
use crate::error::ConfigError;

/// 一个 OpenAI 兼容端点。加提供商 = 加一个 key。
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiCompatibleProvider {
    /// 已包含版本段，例如 `https://minekuai.cn/v1`（代码只拼 `/chat/completions`）。
    pub api_url: String,
    /// 鉴权方式；省略等价于 Bearer + `{ID}_API_KEY`。
    #[serde(default)]
    pub auth: AuthConfig,
    /// 额外 header，同名覆盖默认值。
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// 模型表。顺序保留（以后模型选择器按配置顺序展示）。
    #[serde(default)]
    pub available_models: Vec<AvailableModel>,
}

/// 鉴权方式。密钥本身永远不在配置里，只有变量名。
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AuthConfig {
    /// `Authorization: Bearer <key>`。
    Bearer {
        /// 按顺序取第一个有值的；省略则用推导名（见 [`derived_env_name`]）。
        #[serde(default)]
        env: Vec<String>,
    },
    /// 自定义鉴权 header（minekuai 的 `x-api-key`、Azure 的 `api-key`）。
    Header {
        /// header 名，例如 `x-api-key`。
        name: String,
        /// 按顺序取第一个有值的；省略则用推导名。
        #[serde(default)]
        env: Vec<String>,
    },
    /// 不发鉴权（本地 Ollama / vLLM）。
    None,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self::Bearer { env: Vec::new() }
    }
}

impl AuthConfig {
    fn env_names(&self, provider: &str) -> Vec<String> {
        let declared = match self {
            Self::Bearer { env } | Self::Header { env, .. } => env.as_slice(),
            Self::None => return Vec::new(),
        };
        if declared.is_empty() {
            vec![derived_env_name(provider)]
        } else {
            declared.to_vec()
        }
    }
}

/// 由 provider id 推导环境变量名（对齐 Zed）：
/// `minekuai` → `MINEKUAI_API_KEY`，`my-provider` → `MY_PROVIDER_API_KEY`。
pub fn derived_env_name(provider: &str) -> String {
    let mut name = String::with_capacity(provider.len() + "_API_KEY".len());
    for ch in provider.chars() {
        if ch.is_ascii_alphanumeric() {
            name.push(ch.to_ascii_uppercase());
        } else {
            name.push('_');
        }
    }
    name.push_str("_API_KEY");
    name
}

impl OpenAiCompatibleProvider {
    /// 解析凭证并构造库层客户端。
    pub fn resolve(
        &self,
        id: &str,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, ConfigError> {
        build_provider(
            ProviderSpec {
                id,
                api_url: &self.api_url,
                auth: &self.auth,
                headers: &self.headers,
                available_models: &self.available_models,
            },
            credentials,
        )
    }
}

/// 原生 Ollama 端点（`language_models.ollama`）。
///
/// 形状对齐 Zed `OllamaSettingsContent`：`api_url` 写**根地址**
/// （默认 `http://localhost:11434`），不是 `/v1`。
///
/// 库层还没有原生 `/api/chat` 适配器，所以当前是**过渡实现**：
/// 解析时把根地址补成 OpenAI 兼容的 `/v1`（已带 `/v1` 不重复追加）。
/// 等 `aa_ai_ollama` 落地后，同一份配置会切到原生协议，用户不用改文件。
#[derive(Debug, Clone, Deserialize)]
pub struct OllamaProvider {
    /// 根地址，例如 `http://localhost:11434`。
    #[serde(default = "default_ollama_api_url")]
    pub api_url: String,
    /// 鉴权方式；默认不发（本地 Ollama）。反向代理后面可以用 bearer/header。
    #[serde(default = "no_auth")]
    pub auth: AuthConfig,
    /// 额外 header。Zed 叫 `custom_headers`，两个名字都接受。
    #[serde(default, alias = "custom_headers")]
    pub headers: BTreeMap<String, String>,
    /// 模型表；条目形状与 `openai_compatible` 相同。
    #[serde(default)]
    pub available_models: Vec<AvailableModel>,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self {
            api_url: default_ollama_api_url(),
            auth: AuthConfig::None,
            headers: BTreeMap::new(),
            available_models: Vec::new(),
        }
    }
}

impl OllamaProvider {
    /// 过渡实现用的 OpenAI 兼容基址：`http://localhost:11434` → `.../v1`。
    pub fn openai_base_url(&self) -> String {
        let trimmed = self.api_url.trim().trim_end_matches('/');
        if trimmed.ends_with("/v1") {
            trimmed.to_owned()
        } else {
            format!("{trimmed}/v1")
        }
    }

    /// 解析凭证并构造库层客户端（当前走 OpenAI 兼容协议）。
    pub fn resolve(&self, credentials: &dyn Credentials) -> Result<ResolvedProvider, ConfigError> {
        if self.api_url.trim().is_empty() {
            return Err(ConfigError::Provider {
                provider: OLLAMA_PROVIDER_ID.to_owned(),
                message: "api_url 不能为空".to_owned(),
            });
        }

        build_provider(
            ProviderSpec {
                id: OLLAMA_PROVIDER_ID,
                api_url: &self.openai_base_url(),
                auth: &self.auth,
                headers: &self.headers,
                available_models: &self.available_models,
            },
            credentials,
        )
    }
}

fn default_ollama_api_url() -> String {
    "http://localhost:11434".to_owned()
}

fn no_auth() -> AuthConfig {
    AuthConfig::None
}

/// `language_models.ollama` 的固定 provider id（section 名就是 id）。
pub const OLLAMA_PROVIDER_ID: &str = "ollama";

/// 两种 section 解析成客户端时共用的输入。
struct ProviderSpec<'a> {
    id: &'a str,
    api_url: &'a str,
    auth: &'a AuthConfig,
    headers: &'a BTreeMap<String, String>,
    available_models: &'a [AvailableModel],
}

fn build_provider(
    spec: ProviderSpec<'_>,
    credentials: &dyn Credentials,
) -> Result<ResolvedProvider, ConfigError> {
    let ProviderSpec {
        id,
        api_url,
        auth,
        headers,
        available_models,
    } = spec;

    if api_url.trim().is_empty() {
        return Err(ConfigError::Provider {
            provider: id.to_owned(),
            message: "api_url 不能为空".to_owned(),
        });
    }

    let env_names = auth.env_names(id);
    let client = match auth {
        AuthConfig::None => OpenAiLanguageModelProvider::without_auth(),
        AuthConfig::Bearer { .. } => {
            let key = resolve_key(id, &env_names, credentials)?;
            OpenAiLanguageModelProvider::new(key)
        }
        AuthConfig::Header { name, .. } => {
            if name.trim().is_empty() {
                return Err(ConfigError::Provider {
                    provider: id.to_owned(),
                    message: "auth.kind=header 时 name 不能为空".to_owned(),
                });
            }
            let key = resolve_key(id, &env_names, credentials)?;
            // header 鉴权不走 Bearer；非法 header 名/值在发请求前会变成
            // LlmError::Transport，不会 panic。
            OpenAiLanguageModelProvider::without_auth().with_auth_header(name.clone(), key)
        }
    };

    let mut client = client.with_base_url(api_url.to_owned());
    for (name, value) in headers {
        client = client.with_header(name.clone(), value.clone());
    }

    let mut models = BTreeMap::new();
    for entry in available_models {
        if !entry.capabilities.chat_completions {
            return Err(ConfigError::UnsupportedModel {
                provider: id.to_owned(),
                model: entry.name.clone(),
                reason: "capabilities.chat_completions=false（Responses API 尚未支持）".to_owned(),
            });
        }

        let resolved = ResolvedModel {
            model: entry.to_model(id),
            reasoning_effort: entry.reasoning_effort,
        };
        if models.insert(entry.name.clone(), resolved).is_some() {
            return Err(ConfigError::DuplicateModel {
                provider: id.to_owned(),
                model: entry.name.clone(),
            });
        }
    }

    Ok(ResolvedProvider {
        id: id.to_owned(),
        base_url: api_url.to_owned(),
        client,
        models,
    })
}

fn resolve_key(
    provider: &str,
    names: &[String],
    credentials: &dyn Credentials,
) -> Result<String, ConfigError> {
    for name in names {
        if let Some(value) = credentials.get(provider, name) {
            return Ok(value);
        }
    }
    Err(ConfigError::MissingCredential {
        provider: provider.to_owned(),
        names: names.to_vec(),
    })
}

/// 解析完成的 provider：可以直接交给 agent loop。
pub struct ResolvedProvider {
    pub id: String,
    pub base_url: String,
    pub client: OpenAiLanguageModelProvider,
    models: BTreeMap<String, ResolvedModel>,
}

struct ResolvedModel {
    model: Model,
    reasoning_effort: Option<ReasoningEffort>,
}

impl ResolvedProvider {
    /// 配置里声明的模型（按 id 排序）。
    pub fn models(&self) -> impl Iterator<Item = &Model> {
        self.models.values().map(|entry| &entry.model)
    }

    /// 取模型事实；配置里没声明时返回默认事实，不报错。
    ///
    /// 这样兼容端点临时上线的模型不用改配置就能试。
    pub fn model(&self, id: &str) -> Model {
        self.models
            .get(id)
            .map(|entry| entry.model.clone())
            .unwrap_or_else(|| Model::new(self.id.as_str(), id))
    }

    /// 配置里给这个模型写的默认推理强度。
    ///
    /// 这是**每请求选择**（`Request::reasoning_effort`）的默认值，不是模型事实，
    /// 所以不进 `Model`（见 aa-ai ADR 0006 D4）。
    pub fn reasoning_effort(&self, id: &str) -> Option<ReasoningEffort> {
        self.models.get(id).and_then(|entry| entry.reasoning_effort)
    }

    /// 配置里是否显式声明了这个模型。
    pub fn declares_model(&self, id: &str) -> bool {
        self.models.contains_key(id)
    }
}

impl std::fmt::Debug for ResolvedProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 刻意不打印 client：它持有密钥。
        f.debug_struct("ResolvedProvider")
            .field("id", &self.id)
            .field("base_url", &self.base_url)
            .field("models", &self.models.keys().collect::<Vec<_>>())
            .finish()
    }
}

/// 一个模型条目。
#[derive(Debug, Clone, Deserialize)]
pub struct AvailableModel {
    /// 端点上的模型 id（wire 的 `model`）。
    pub name: String,
    /// 给人看的名字；暂未使用，留给 UI。
    pub display_name: Option<String>,
    /// 上下文窗口（输入 + 输出）。Zed 的 `max_tokens` 是同一个意思。
    pub context_window: Option<u64>,
    /// 输出上限。
    pub max_output_tokens: Option<u64>,
    /// 默认推理强度；发起请求时填进 `Request::reasoning_effort`。
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(default)]
    pub capabilities: Capabilities,
}

impl AvailableModel {
    fn to_model(&self, provider: &str) -> Model {
        let mut model = Model::new(provider, self.name.as_str());

        model.context_window = self.context_window;
        model.max_output_tokens = self.max_output_tokens;

        model.supports_tools = self.capabilities.tools;
        model.supports_images = self.capabilities.images;
        model.supports_parallel_tool_calls = self.capabilities.parallel_tool_calls;
        model.supports_prompt_cache_key = self.capabilities.prompt_cache_key;
        // `max_tokens_parameter=true` 表示发 `max_tokens`，所以
        // `uses_max_completion_tokens` 是它的反。
        model.uses_max_completion_tokens = !self.capabilities.max_tokens_parameter;

        model
    }
}

/// 模型能力位。默认值与 Zed `OpenAiCompatibleModelCapabilities::default()` 一致。
#[derive(Debug, Clone, Deserialize)]
pub struct Capabilities {
    /// 接受 `tools` / `tool_choice`。
    #[serde(default = "default_true")]
    pub tools: bool,
    /// 接受图片输入。
    #[serde(default)]
    pub images: bool,
    /// 接受 `parallel_tool_calls`。
    #[serde(default)]
    pub parallel_tool_calls: bool,
    /// 接受 `prompt_cache_key`。
    #[serde(default)]
    pub prompt_cache_key: bool,
    /// 端点支持 `/chat/completions`；`false` 表示只有 Responses API（尚未支持）。
    #[serde(default = "default_true")]
    pub chat_completions: bool,
    /// 思考与工具调用交错（决定回放方式；目前只保存在配置里）。
    #[serde(default)]
    pub interleaved_reasoning: bool,
    /// `true` 发 `max_tokens`，`false` 发 `max_completion_tokens`。
    #[serde(default)]
    pub max_tokens_parameter: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            tools: true,
            images: false,
            parallel_tool_calls: false,
            prompt_cache_key: false,
            chat_completions: true,
            interleaved_reasoning: false,
            max_tokens_parameter: false,
        }
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_env_names_like_zed() {
        assert_eq!(derived_env_name("minekuai"), "MINEKUAI_API_KEY");
        assert_eq!(derived_env_name("my-provider"), "MY_PROVIDER_API_KEY");
        assert_eq!(derived_env_name("Agnes_2"), "AGNES_2_API_KEY");
    }
}
