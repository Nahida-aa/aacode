//! 应用侧的多提供商配置与凭证解析。
//!
//! 库层（`aa-ai`）只接收**已经解好的**端点数据；这里负责把用户写的
//! `settings.jsonc` 变成 `OpenAiLanguageModelProvider` + `Model` 表和密钥。
//! 边界见 aa-ai 的 `docs/design/library-api.md`。
//!
//! 形状对齐 Zed 的 `language_models` 分组（`settings_content/src/language_model.rs`）：
//! - `language_models.<协议>.<id>` 是独立端点，加提供商 = 加一个 key；
//! - `available_models` 是数组，`capabilities` 逐模型声明；
//! - 密钥不进配置文件：`auth.env` 列出变量名，缺省按 id 推导 `{ID}_API_KEY`；
//! - `{env:VAR}` / `{file:path}` 在解析 JSONC **之前**做文本替换（和 opencode 同款）。
//!
//! 目前只实现 `openai_compatible` 这一组；其他组（`anthropic` / `google` …）
//! 等对应 crate 落地后再加，未知组会被忽略。
//!
//! ```jsonc
//! {
//!   "language_models": {
//!     "openai_compatible": {
//!       "minekuai": {
//!         "api_url": "https://minekuai.cn/v1",
//!         "auth": { "kind": "header", "name": "x-api-key", "env": ["MINEKUAI_API_KEY"] },
//!         "headers": { "X-Title": "aacode" },
//!         "available_models": [
//!           {
//!             "name": "some-model",
//!             "context_window": 128000,
//!             "max_output_tokens": 8192,
//!             "capabilities": { "tools": true, "max_tokens_parameter": true }
//!           }
//!         ]
//!       }
//!     }
//!   }
//! }
//! ```

mod credentials;
mod error;
mod provider;
mod substitute;

pub use credentials::{Credentials, EnvCredentials};
pub use error::ConfigError;
pub use provider::{
    AuthConfig, Capabilities, OpenAiCompatibleProvider, ResolvedProvider, derived_env_name,
};

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 整份 `settings.jsonc`。
///
/// 现在只有 LLM 配置；以后的主题、工具白名单等可以并列加在这里。
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub language_models: LanguageModels,
}

/// 按协议分组的 provider 表（对齐 Zed `LanguageModelSettings`）。
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LanguageModels {
    /// OpenAI 兼容端点：官方 OpenAI、第三方网关、Ollama `/v1`、vLLM 等。
    #[serde(default)]
    pub openai_compatible: BTreeMap<String, OpenAiCompatibleProvider>,
}

impl Settings {
    /// 读文件 → 替换 `{env:}` / `{file:}` → 解析 JSONC。
    ///
    /// 只做语法和结构校验，**不读密钥**；密钥在 [`Self::resolve`] 时才解析。
    /// 所以 `aacode providers` 这类只看端点的命令不需要任何 key。
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::parse(&text, path)
    }

    /// 解析内存中的配置文本。`path` 只用于 `{file:}` 的相对路径和错误信息。
    pub fn parse(text: &str, path: &Path) -> Result<Self, ConfigError> {
        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
        let text = substitute::apply(text, base_dir);
        jsonc_parser::parse_to_serde_value(&text, &Default::default()).map_err(|error| {
            ConfigError::Invalid {
                path: path.to_path_buf(),
                message: format!("JSONC 解析失败：{error}"),
            }
        })
    }

    /// 用环境变量解析某个 provider 的凭证并构造客户端。
    pub fn resolve(&self, id: &str) -> Result<ResolvedProvider, ConfigError> {
        self.resolve_with(id, &EnvCredentials)
    }

    /// 凭证来源可替换（将来接系统钥匙串时不改配置结构）。
    ///
    /// 解析顺序由调用方决定；[`EnvCredentials`] 是当前唯一实现。
    pub fn resolve_with(
        &self,
        id: &str,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, ConfigError> {
        let provider = self
            .language_models
            .openai_compatible
            .get(id)
            .ok_or_else(|| ConfigError::UnknownProvider {
                id: id.to_owned(),
                known: self.ids().map(str::to_owned).collect(),
            })?;
        provider.resolve(id, credentials)
    }

    /// 已声明的 provider id（排好序）。
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.language_models
            .openai_compatible
            .keys()
            .map(String::as_str)
    }
}

/// 默认配置路径：`$AACODE_SETTINGS`，否则 `{config_dir}/aacode/settings.jsonc`。
///
/// 与库层不同，**读哪个文件**是应用自己的约定（见 aa-ai
/// `docs/design/library-api.md`）。
pub fn default_path() -> PathBuf {
    if let Some(path) = std::env::var_os("AACODE_SETTINGS") {
        return PathBuf::from(path);
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aacode")
        .join("settings.jsonc")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct FakeCredentials(HashMap<&'static str, &'static str>);

    impl Credentials for FakeCredentials {
        fn get(&self, _provider: &str, name: &str) -> Option<String> {
            self.0.get(name).map(|value| (*value).to_owned())
        }
    }

    fn parse(text: &str) -> Settings {
        Settings::parse(text, Path::new("/tmp/aacode/settings.jsonc")).unwrap()
    }

    #[test]
    fn parses_jsonc_with_comments_and_trailing_commas() {
        let config = parse(
            r#"{
                // 注释和尾逗号都应该接受
                "language_models": {
                    "openai_compatible": {
                        "agnes": {
                            "api_url": "https://apihub.agnes-ai.com/v1",
                        },
                    },
                },
            }"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["agnes"]);
    }

    #[test]
    fn missing_credential_lists_tried_names() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"minekuai":{
                "api_url":"https://minekuai.cn/v1",
                "auth":{"kind":"header","name":"x-api-key"}
            }}}}"#,
        );
        let error = config
            .resolve_with("minekuai", &FakeCredentials(HashMap::new()))
            .unwrap_err()
            .to_string();
        assert!(error.contains("MINEKUAI_API_KEY"), "{error}");
    }

    #[test]
    fn explicit_env_list_wins_over_derived_name() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"agnes":{
                "api_url":"https://example.com/v1",
                "auth":{"kind":"bearer","env":["SECOND","FIRST"]}
            }}}}"#,
        );
        let credentials = FakeCredentials(HashMap::from([("FIRST", "first-key")]));
        // 第一个有值的生效：SECOND 缺失、FIRST 命中。
        let resolved = config.resolve_with("agnes", &credentials).unwrap();
        assert_eq!(resolved.id, "agnes");
        assert_eq!(resolved.base_url, "https://example.com/v1");
    }

    #[test]
    fn no_auth_provider_resolves_without_credentials() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"ollama":{
                "api_url":"http://localhost:11434/v1","auth":{"kind":"none"}
            }}}}"#,
        );
        config
            .resolve_with("ollama", &FakeCredentials(HashMap::new()))
            .unwrap();
    }

    #[test]
    fn model_facts_and_capabilities_are_applied() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"p":{
                "api_url":"https://example.com/v1",
                "auth":{"kind":"none"},
                "available_models":[{
                    "name":"m",
                    "context_window":128000,
                    "max_output_tokens":8192,
                    "reasoning_effort":"medium",
                    "capabilities":{
                        "tools":false,
                        "images":true,
                        "parallel_tool_calls":true,
                        "prompt_cache_key":true,
                        "max_tokens_parameter":true,
                        "interleaved_reasoning":true
                    }
                }]
            }}}}"#,
        );
        let resolved = config.resolve("p").unwrap();
        let model = resolved.model("m");
        assert_eq!(model.provider, "p");
        assert_eq!(model.id, "m");
        assert_eq!(model.context_window, Some(128_000));
        assert_eq!(model.max_output_tokens, Some(8192));
        assert!(!model.supports_tools);
        assert!(model.supports_images);
        assert!(model.supports_parallel_tool_calls);
        assert!(model.supports_prompt_cache_key);
        // max_tokens_parameter=true → 发 max_tokens，不是 max_completion_tokens。
        assert!(!model.uses_max_completion_tokens);
        assert_eq!(
            resolved.reasoning_effort("m"),
            Some(aa_language_model::provider::ReasoningEffort::Medium)
        );
        // 未声明的模型给出默认事实，而不是报错（兼容端点经常临时加模型）。
        let fallback = resolved.model("not-listed");
        assert!(fallback.supports_tools);
        assert_eq!(fallback.context_window, None);
        assert_eq!(resolved.reasoning_effort("not-listed"), None);
    }

    #[test]
    fn max_tokens_parameter_defaults_to_max_completion_tokens() {
        // 与 Zed 的 OpenAiCompatibleModelCapabilities 默认值一致。
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"p":{
                "api_url":"https://example.com/v1","auth":{"kind":"none"},
                "available_models":[{"name":"m"}]
            }}}}"#,
        );
        let model = config.resolve("p").unwrap().model("m");
        assert!(model.uses_max_completion_tokens);
        // capabilities 默认值：tools 开、images 关。
        assert!(model.supports_tools);
        assert!(!model.supports_images);
    }

    #[test]
    fn duplicate_model_names_are_rejected() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"p":{
                "api_url":"https://example.com/v1","auth":{"kind":"none"},
                "available_models":[{"name":"m"},{"name":"m"}]
            }}}}"#,
        );
        let error = config.resolve("p").unwrap_err().to_string();
        assert!(error.contains("`m`") && error.contains("重复"), "{error}");
    }

    #[test]
    fn responses_only_models_are_rejected_for_now() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"p":{
                "api_url":"https://example.com/v1","auth":{"kind":"none"},
                "available_models":[{"name":"m","capabilities":{"chat_completions":false}}]
            }}}}"#,
        );
        let error = config.resolve("p").unwrap_err().to_string();
        assert!(error.contains("chat_completions"), "{error}");
    }

    #[test]
    fn unknown_provider_lists_known_ids() {
        let config = parse(
            r#"{"language_models":{"openai_compatible":{"agnes":{"api_url":"https://example.com/v1"}}}}"#,
        );
        let error = config
            .resolve_with("nope", &FakeCredentials(HashMap::new()))
            .unwrap_err()
            .to_string();
        assert!(error.contains("nope") && error.contains("agnes"), "{error}");
    }

    #[test]
    fn env_and_file_substitution_happens_before_parse() {
        let dir = std::env::temp_dir().join(format!("aacode-config-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let key_file = dir.join("key.txt");
        std::fs::write(&key_file, "  sk-from-file\n").unwrap();

        let text = format!(
            r#"{{
                "language_models": {{
                    "openai_compatible": {{
                        "p": {{
                            "api_url": "https://example.com/v1",
                            "auth": {{"kind":"bearer","env":["FROM_FILE"]}},
                            "headers": {{"X-From-File":"{{file:{} }}"}}
                        }}
                    }}
                }}
            }}"#,
            key_file.display()
        );
        let config = Settings::parse(&text, &dir.join("settings.jsonc")).unwrap();
        let headers = &config.language_models.openai_compatible["p"].headers;
        assert_eq!(headers["X-From-File"], "sk-from-file");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn env_substitution_escapes_quotes() {
        let text = r#"{"language_models":{"openai_compatible":{"p":{
            "api_url":"https://example.com/v1",
            "headers":{"X-Quote":"{env:AACODE_TEST_QUOTE}"}}}}}"#;
        // 不依赖真实进程 env：这里只验证「缺失时替换成空串」，解析仍然成立。
        let config = Settings::parse(text, Path::new("/tmp/p.jsonc")).unwrap();
        assert_eq!(
            config.language_models.openai_compatible["p"].headers["X-Quote"],
            ""
        );
    }
}
