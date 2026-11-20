//! 端点配置形状 + 解析成库层客户端。
//!
//! 两种 section，名字代表**协议**而不是厂商（对齐 Zed）：
//! - [`OpenAiCompatibleProvider`]：`language_model_providers.openai_compatible.<id>`，走 `/chat/completions`（SSE）；
//! - [`OllamaProvider`]：`language_model_providers.ollama`，走原生 `/api/chat`（NDJSON）。
//!
//! 字段名以 Zed 为参照，但端点基址叫 `base_url`（Zed 叫 `api_url`）：
//! 与库层 `OpenAiLanguageModelProvider::with_base_url`、OpenAI SDK 的
//! `base_url` 一致，语义也更明确（后面拼的是协议路径）。
//!
//! - `base_url` / `available_models` / `headers`；
//! - 模型事实 `context_window`（Zed 叫 `max_tokens`，但那是上下文窗口，不是输出上限）
//!   与 `max_output_tokens`；
//! - `capabilities` 的默认值与 Zed 相同（`tools` 开、`images` 关、
//!   `max_tokens_parameter` 关 → 默认发 `max_completion_tokens`）。
//!
//! `auth` 是 aa-ai 的扩展：Zed 把密钥放钥匙串，这里放「变量名」。

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use aa_ai_ollama::OllamaLanguageModelProvider;
use aa_ai_openai::OpenAiLanguageModelProvider;
use aa_language_model::provider::{Model, ReasoningEffort};
use serde::Deserialize;

use crate::credentials::Credentials;
use crate::error::SettingsError;

/// 一个 OpenAI 兼容端点。加提供商 = 加一个 key。
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiCompatibleProvider {
    /// 已包含版本段，例如 `https://minekuai.cn/v1`（代码只拼 `/chat/completions`）。
    pub base_url: String,
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
    ///
    /// `transport` 是全进程共享的 HTTP 客户端：代理、超时、连接池都在它身上
    /// （`aacode_settings::Settings::transport` 构造并缓存）。
    pub fn resolve(
        &self,
        id: &str,
        transport: &Arc<aa_http_client::HttpClient>,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, SettingsError> {
        build_provider(
            ProviderSpec {
                id,
                base_url: &self.base_url,
                auth: &self.auth,
                headers: &self.headers,
                available_models: &self.available_models,
            },
            transport,
            credentials,
        )
    }
}

/// 原生 Ollama 端点（`language_model_providers.ollama`）。
///
/// 形状对齐 Zed `OllamaSettingsContent`：`base_url` 写**根地址**
/// （默认 `http://localhost:11434`），不是 `/v1`。Zed 把这个字段叫 `api_url`。
///
/// 走的是**原生协议**（`/api/chat`、NDJSON、`thinking` 字段），由
/// `aa_ai_ollama` 实现；和 `openai_compatible.ollama`（`/v1`）是两回事，
/// 不会互相顶替。
#[derive(Debug, Clone, Deserialize)]
pub struct OllamaProvider {
    /// 根地址，例如 `http://localhost:11434`。
    #[serde(default = "default_ollama_base_url")]
    pub base_url: String,
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
            base_url: default_ollama_base_url(),
            auth: AuthConfig::None,
            headers: BTreeMap::new(),
            available_models: Vec::new(),
        }
    }
}

fn default_ollama_base_url() -> String {
    "http://localhost:11434".to_owned()
}

fn no_auth() -> AuthConfig {
    AuthConfig::None
}

impl OllamaProvider {
    /// 解析凭证并构造 `aa_ai_ollama` 的客户端。
    ///
    /// `base_url` 是根地址；库层自己拼 `/api/chat` 与 `/api/tags`。
    pub fn resolve(
        &self,
        transport: &Arc<aa_http_client::HttpClient>,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, SettingsError> {
        build_ollama_provider(self, transport, credentials)
    }
}

/// `language_model_providers.ollama` 的固定 provider id（section 名就是 id）。
pub const OLLAMA_PROVIDER_ID: &str = "ollama";

/// OpenAI 兼容端点解析成客户端时的输入。
struct ProviderSpec<'a> {
    id: &'a str,
    base_url: &'a str,
    auth: &'a AuthConfig,
    headers: &'a BTreeMap<String, String>,
    available_models: &'a [AvailableModel],
}

fn build_provider(
    spec: ProviderSpec<'_>,
    transport: &Arc<aa_http_client::HttpClient>,
    credentials: &dyn Credentials,
) -> Result<ResolvedProvider, SettingsError> {
    let ProviderSpec {
        id,
        base_url,
        auth,
        headers,
        available_models,
    } = spec;

    if base_url.trim().is_empty() {
        return Err(SettingsError::Provider {
            provider: id.to_owned(),
            message: "base_url 不能为空".to_owned(),
        });
    }

    let env_names = auth.env_names(id);
    let client = match auth {
        AuthConfig::None => OpenAiLanguageModelProvider::without_auth(transport.clone()),
        AuthConfig::Bearer { .. } => {
            let key = resolve_key(id, &env_names, credentials)?;
            OpenAiLanguageModelProvider::new(transport.clone(), key)
        }
        AuthConfig::Header { name, .. } => {
            if name.trim().is_empty() {
                return Err(SettingsError::Provider {
                    provider: id.to_owned(),
                    message: "auth.kind=header 时 name 不能为空".to_owned(),
                });
            }
            let key = resolve_key(id, &env_names, credentials)?;
            // header 鉴权不走 Bearer；非法 header 名/值在发请求前会变成
            // LlmError::Transport，不会 panic。
            OpenAiLanguageModelProvider::without_auth(transport.clone())
                .with_auth_header(name.clone(), key)
        }
    };

    let mut client = client.with_base_url(base_url.to_owned());
    for (name, value) in headers {
        client = client.with_header(name.clone(), value.clone());
    }

    let models = build_models(id, available_models, true)?;

    Ok(ResolvedProvider {
        id: id.to_owned(),
        base_url: base_url.to_owned(),
        client: ProviderClient::OpenAi(client),
        models,
    })
}

/// 原生 Ollama：根地址 + 可选的鉴权（反向代理后面才需要）+ 模型表。
fn build_ollama_provider(
    provider: &OllamaProvider,
    transport: &Arc<aa_http_client::HttpClient>,
    credentials: &dyn Credentials,
) -> Result<ResolvedProvider, SettingsError> {
    let id = OLLAMA_PROVIDER_ID;
    let base_url = provider.base_url.trim();
    if base_url.is_empty() {
        return Err(SettingsError::Provider {
            provider: id.to_owned(),
            message: "base_url 不能为空".to_owned(),
        });
    }

    let mut client = match &provider.auth {
        AuthConfig::None => OllamaLanguageModelProvider::new(transport.clone()),
        AuthConfig::Bearer { .. } => {
            let names = provider.auth.env_names(id);
            // 原生 Ollama 默认无鉴权；写了 bearer 却没有 key 才算配置错误。
            let token = resolve_key(id, &names, credentials)?;
            OllamaLanguageModelProvider::new(transport.clone()).with_bearer_token(token)
        }
        AuthConfig::Header { name, .. } => {
            if name.trim().is_empty() {
                return Err(SettingsError::Provider {
                    provider: id.to_owned(),
                    message: "auth.kind=header 时 name 不能为空".to_owned(),
                });
            }
            let names = provider.auth.env_names(id);
            let value = resolve_key(id, &names, credentials)?;
            OllamaLanguageModelProvider::new(transport.clone())
                .with_auth_header(name.clone(), value)
        }
    };

    client = client.with_base_url(base_url.to_owned());
    for (name, value) in &provider.headers {
        client = client.with_header(name.clone(), value.clone());
    }

    // `capabilities.chat_completions` 是「这个模型能不能走 OpenAI `/v1`」，
    // 对原生 section 没有意义，所以不校验。
    let models = build_models(id, &provider.available_models, false)?;

    Ok(ResolvedProvider {
        id: id.to_owned(),
        base_url: base_url.to_owned(),
        client: ProviderClient::Ollama(client),
        models,
    })
}

/// 模型表：wire 形状无关，两种协议共用。
///
/// `require_chat_completions` 只对 OpenAI 兼容 section 有意义（`false` 表示
/// 「这个模型只能走 Responses API」→ 明确报错，而不是偷偷换协议）。
fn build_models(
    id: &str,
    available_models: &[AvailableModel],
    require_chat_completions: bool,
) -> Result<BTreeMap<String, ResolvedModel>, SettingsError> {
    let mut models = BTreeMap::new();
    for entry in available_models {
        if require_chat_completions && !entry.capabilities.chat_completions {
            return Err(SettingsError::UnsupportedModel {
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
            return Err(SettingsError::DuplicateModel {
                provider: id.to_owned(),
                model: entry.name.clone(),
            });
        }
    }
    Ok(models)
}

fn resolve_key(
    provider: &str,
    names: &[String],
    credentials: &dyn Credentials,
) -> Result<String, SettingsError> {
    for name in names {
        if let Some(value) = credentials.get(provider, name) {
            return Ok(value);
        }
    }
    Err(SettingsError::MissingCredential {
        provider: provider.to_owned(),
        names: names.to_vec(),
    })
}

/// 解析完成的 provider：可以直接交给 agent loop。
pub struct ResolvedProvider {
    pub id: String,
    pub base_url: String,
    pub client: ProviderClient,
    models: BTreeMap<String, ResolvedModel>,
}

/// 解析完成的库层客户端：按**协议**分派。
///
/// 两种协议的 wire 形状完全不同（SSE vs NDJSON、`reasoning_content` vs
/// `thinking`、工具参数是字符串 vs 对象），库层因此是两个具体类型。应用的其他
/// 部分只关心 `LanguageModelClient` 与 `list_models`，所以在这里收成枚举：
/// 加一种协议 = 加一个变体，调用点不用改。
pub enum ProviderClient {
    /// `aa_ai_openai`：`/chat/completions`。
    OpenAi(OpenAiLanguageModelProvider),
    /// `aa_ai_ollama`：原生 `/api/chat`。
    Ollama(OllamaLanguageModelProvider),
}

/// `list_models` 的归一化结果：两套 wire 形状共有的部分。
///
/// OpenAI 是 `{"data":[{"id","owned_by"}]}`，Ollama 是
/// `{"models":[{"name","details":{"family"}}]}`；这里只留「能填进 `Model::id` 的
/// 名字」和可选的归属/家族。
#[derive(Debug, Clone)]
pub struct ModelSummary {
    pub id: String,
    pub owned_by: Option<String>,
}

impl ProviderClient {
    /// 列出端点声明的模型。
    ///
    /// 这不是 trait 方法：`LanguageModelClient` 只负责「发一次对话请求」，
    /// 模型目录是端点能力，不是协议语义。
    pub async fn list_models(
        &self,
    ) -> Result<Vec<ModelSummary>, aa_language_model::provider::LlmError> {
        match self {
            Self::OpenAi(client) => client.list_models().await.map(|models| {
                models
                    .into_iter()
                    .map(|model| ModelSummary {
                        id: model.id,
                        owned_by: model.owned_by,
                    })
                    .collect()
            }),
            Self::Ollama(client) => client.list_models().await.map(|models| {
                models
                    .into_iter()
                    .map(|model| ModelSummary {
                        id: model.name,
                        owned_by: model.details.and_then(|details| details.family),
                    })
                    .collect()
            }),
        }
    }
}

impl aa_language_model::provider::LanguageModelClient for ProviderClient {
    fn stream_completion(
        &self,
        model: &Model,
        request: &aa_language_model::request::Request,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        aa_language_model::provider::EventStream,
                        aa_language_model::provider::LlmError,
                    >,
                > + Send
                + 'static,
        >,
    > {
        match self {
            Self::OpenAi(client) => client.stream_completion(model, request),
            Self::Ollama(client) => client.stream_completion(model, request),
        }
    }
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
