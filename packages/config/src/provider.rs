//! 单个 provider 的配置形状 + 解析成库层客户端。

use std::collections::BTreeMap;

use aa_ai_openai::OpenAiLanguageModelProvider;
use aa_language_model::provider::Model;
use serde::Deserialize;

use crate::credentials::Credentials;
use crate::error::ConfigError;

/// 一个 provider 端点。加提供商 = 加一个 key（对应 opencode 的 `provider.<id>`、
/// Zed 的 `openai_compatible.<id>`）。
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    /// 已包含版本段，例如 `https://minekuai.cn/v1`（代码只拼 `/chat/completions`）。
    pub base_url: String,
    /// 鉴权方式；省略等价于 Bearer + `{ID}_API_KEY`。
    #[serde(default)]
    pub auth: AuthConfig,
    /// 额外 header，同名覆盖默认值。
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// 模型事实表；模型名不必在这里声明也能用（会拿到默认事实）。
    #[serde(default)]
    pub models: BTreeMap<String, ModelConfig>,
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

impl ProviderConfig {
    /// 解析凭证并构造库层客户端。
    pub fn resolve(
        &self,
        id: &str,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, ConfigError> {
        if self.base_url.trim().is_empty() {
            return Err(ConfigError::Provider {
                provider: id.to_owned(),
                message: "base_url 不能为空".to_owned(),
            });
        }

        let env_names = self.auth.env_names(id);
        let client = match &self.auth {
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

        let mut client = client.with_base_url(self.base_url.clone());
        for (name, value) in &self.headers {
            client = client.with_header(name.clone(), value.clone());
        }

        let models = self
            .models
            .iter()
            .map(|(model_id, config)| (model_id.clone(), config.to_model(id, model_id)))
            .collect();

        Ok(ResolvedProvider {
            id: id.to_owned(),
            base_url: self.base_url.clone(),
            client,
            models,
        })
    }
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
    models: BTreeMap<String, Model>,
}

impl ResolvedProvider {
    /// 配置里声明的模型（按 id 排序）。
    pub fn models(&self) -> impl Iterator<Item = &Model> {
        self.models.values()
    }

    /// 取模型事实；配置里没声明时返回默认事实，不报错。
    ///
    /// 这样兼容端点临时上线的模型不用改配置就能试。
    pub fn model(&self, id: &str) -> Model {
        self.models
            .get(id)
            .cloned()
            .unwrap_or_else(|| Model::new(self.id.as_str(), id))
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

/// 一个模型的事实。字段与 `aa_language_model::provider::Model` 对齐；
/// 省略的字段用 `Model::new` 的默认值。
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ModelConfig {
    /// 上下文窗口（输入 + 输出）。
    pub context_window: Option<u64>,
    /// 输出上限。
    pub max_output_tokens: Option<u64>,
    /// `true` 发 `max_completion_tokens`（o 系列 / GPT-5）。
    pub uses_max_completion_tokens: Option<bool>,
    /// `true` 把 `system` 发成 `developer`。
    pub uses_developer_role: Option<bool>,
    /// 是否接受 `tools` / `tool_choice`。
    pub supports_tools: Option<bool>,
    /// 是否接受 `parallel_tool_calls`。
    pub supports_parallel_tool_calls: Option<bool>,
    /// 是否接受图片输入。
    pub supports_images: Option<bool>,
    /// 是否接受 `prompt_cache_key`。
    pub supports_prompt_cache_key: Option<bool>,
}

impl ModelConfig {
    pub fn to_model(&self, provider: &str, id: &str) -> Model {
        let mut model = Model::new(provider, id);

        if let Some(value) = self.uses_max_completion_tokens {
            model.uses_max_completion_tokens = value;
        }
        if let Some(value) = self.uses_developer_role {
            model.uses_developer_role = value;
        }
        if let Some(value) = self.supports_tools {
            model.supports_tools = value;
        }
        if let Some(value) = self.supports_parallel_tool_calls {
            model.supports_parallel_tool_calls = value;
        }
        if let Some(value) = self.supports_images {
            model.supports_images = value;
        }
        if let Some(value) = self.supports_prompt_cache_key {
            model.supports_prompt_cache_key = value;
        }
        model.context_window = self.context_window;
        model.max_output_tokens = self.max_output_tokens;

        model
    }
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
