//! 应用侧的多提供商设置与凭证解析。
//!
//! 库层（`aa-ai`）只接收**已经解好的**端点数据；这里负责把用户写的
//! `settings.jsonc` 变成 provider 客户端 + `Model` 表和密钥。
//! 边界见 aa-ai 的 `docs/design/library-api.md`。
//!
//! 形状参考 Zed 的 `language_models` 分组（`settings_content/src/language_model.rs`），
//! 但 key 叫 `language_model_providers`（这里只放 provider，不放模型选择）：
//! - section 名代表**协议**，或一家/一个本地服务差异大到必须单列的厂商：
//!   `openai_compatible.<id>` 走 `/chat/completions`（SSE），
//!   `ollama` 走原生 `/api/chat`（NDJSON，适配器是 `aa_ai_ollama`），
//!   `llama.cpp` 是独立 section（wire 是 OpenAI 兼容 `/chat/completions`，
//!   但默认地址 / 无鉴权归这个 section），
//!   `openrouter` 走 OpenRouter 的 `/chat/completions`（适配器是 `aa_ai_openrouter`）；
//! - `available_models` 是数组，`capabilities` 逐模型声明；
//! - 密钥不进配置文件：`auth.env` 列出变量名，缺省按 id 推导 `{ID}_API_KEY`；
//! - `{env:VAR}` / `{file:path}` 在解析 JSONC **之前**做文本替换（和 opencode 同款）；
//! - 顶层 `proxy`（对齐 Zed 同名设置）：http/https 代理，空串 = 读环境变量，
//!   `NO_PROXY` 的直连列表始终生效；socks 代理需要 reqwest 的 `socks` feature（未启用）。
//!
//! 每个 section 都有适配器：`openai_compatible`（`/v1`）、`ollama`（原生
//! `/api/chat`）、`llama.cpp`（独立 section，内部仍用 OpenAI 兼容客户端）
//! 与 `openrouter`。同名 id（例如都叫 `ollama`）时
//! **`openai_compatible` 覆盖内置 section**，与 Zed 的注册顺序一致。
//! 其他 section（`deepseek` …）会被收进 [`Settings::unsupported_sections`]，
//! CLI 会提示「已忽略」，避免写了设置却完全没反应。
//!
//! # 默认值与合并
//!
//! [`Settings::load`] 先读内置的 `assets/settings/default.jsonc`，再把用户文件深合并上去
//! （规则对齐 Zed `merge_from.rs`）：
//!
//! - 对象逐键递归合并；
//! - 用户写 `null` 或省略字段 = 保留默认值；
//! - 数组整体覆盖，不做逐项合并。
//!
//! 所以 `"ollama": null` 是合法的，含义是「不覆盖默认的 ollama」。
//! 没有默认层时（[`Settings::parse`]）`null` 同样视为「没写」，条目直接不存在。
//!
//! ```jsonc
//! {
//!   "language_model_providers": {
//!     "openai_compatible": {
//!       "minekuai": {
//!         "base_url": "https://minekuai.cn/v1",
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
pub use error::SettingsError;
pub use provider::{
    AuthConfig, AvailableModel, Capabilities, DEFAULT_LLAMA_CPP_BASE_URL, LLAMA_CPP_PROVIDER_ID,
    LocalOpenAiProvider, OLLAMA_PROVIDER_ID, OPENROUTER_PROVIDER_ID, OllamaProvider,
    OpenAiCompatibleProvider, OpenRouterProvider, ResolvedProvider, derived_env_name,
};

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// 内置默认设置（只放不需要密钥的本地端点）。
const DEFAULT_SETTINGS: &str = include_str!("../../../assets/settings/default.jsonc");

/// 整份 `settings.jsonc`。
///
/// 现在只有 LLM 配置；以后的主题、工具白名单等可以并列加在这里。
#[derive(Clone, Default, serde::Deserialize)]
pub struct Settings {
    /// 出站 HTTP/HTTPS 代理。形状与语义对齐 Zed 的顶层 `proxy` 设置：
    ///
    /// - 空串或省略 = 不显式设置，交给 HTTP 客户端读环境变量
    ///   （`ALL_PROXY` / `HTTPS_PROXY` / `HTTP_PROXY`）；
    /// - 非空 = 覆盖环境变量代理，`NO_PROXY` 的直连列表仍然生效；
    /// - 只接受 `http://` / `https://`（socks 需要 reqwest 的 `socks` feature，未启用）。
    #[serde(default)]
    pub proxy: Option<String>,
    #[serde(default)]
    pub language_model_providers: LanguageModelProviders,
    /// 用户文件里出现、但还没有适配器的 section。只用于提示，
    /// 不影响解析（内置默认文件里也有这类占位）。
    #[serde(skip)]
    unsupported: BTreeSet<String>,
    /// 全进程共享的 HTTP 传输（一个连接池）。按 `proxy` 首次使用时构造。
    ///
    /// 放在设置层是因为「代理怎么走」是应用配置；库层只接收构造好的
    /// `Arc<dyn aa_http_client::HttpClient>`（见 aa-ai `docs/decisions/0010-transport-object.md`）。
    #[serde(skip)]
    transport: OnceLock<Arc<dyn aa_http_client::HttpClient>>,
}

// `dyn HttpClient` 没有 `Debug`（trait 是跨 crate 的，不能补 impl），手写一个：
// 传输只报「是否已构造」，不打印连接池内部状态。
impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("proxy", &self.proxy)
            .field("language_model_providers", &self.language_model_providers)
            .field("unsupported", &self.unsupported)
            .field("transport", &self.transport.get().map(|_| "<shared>"))
            .finish()
    }
}

/// 按协议分组的 provider 表（对齐 Zed `LanguageModelSettings`）。
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LanguageModelProviders {
    /// OpenAI 兼容端点：官方 OpenAI、第三方网关、vLLM 等。
    #[serde(default)]
    pub openai_compatible: BTreeMap<String, OpenAiCompatibleProvider>,
    /// 本地 llama.cpp（`llama-server` 的 `/chat/completions`）。独立 section，
    /// 默认无鉴权。
    #[serde(default, rename = "llama.cpp")]
    pub llama_cpp: Option<LocalOpenAiProvider>,
    /// 原生 Ollama 端点（语义上是 `/api/chat`，由 `aa_ai_ollama` 处理）。
    #[serde(default)]
    pub ollama: Option<OllamaProvider>,
    /// OpenRouter（chat completions 同形，但思考回放 / 错误体 / 归因 header 不同，
    /// 由 `aa_ai_openrouter` 处理）。
    #[serde(default)]
    pub openrouter: Option<OpenRouterProvider>,
}

impl Settings {
    /// 读用户文件，深合并到内置默认值上。
    ///
    /// 只做语法和结构校验，**不读密钥**；密钥在 [`Self::resolve`] 时才解析。
    /// 所以 `aacode providers` 这类只看端点的命令不需要任何 key。
    pub fn load(path: &Path) -> Result<Self, SettingsError> {
        let text = std::fs::read_to_string(path).map_err(|source| SettingsError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::load_with_defaults(&text, DEFAULT_SETTINGS, path)
    }

    /// 解析内存中的配置文本，**不合并**内置默认值。
    ///
    /// `null` 仍然等价于「没写」：该条目直接不存在。
    pub fn parse(text: &str, path: &Path) -> Result<Self, SettingsError> {
        Self::load_with_defaults(text, "{}", path)
    }

    /// 解析并合并：`defaults` 作为底，`user` 覆盖上去。
    ///
    /// `path` 只用于 `{file:}` 的相对路径和错误信息。
    pub fn load_with_defaults(
        user: &str,
        defaults: &str,
        path: &Path,
    ) -> Result<Self, SettingsError> {
        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));

        let mut merged = parse_jsonc(defaults, base_dir, path)?;
        let user_value = parse_jsonc(user, base_dir, path)?;
        // 只把**用户写的**未知 section 记下来；默认文件里的占位不提示。
        let unsupported = unknown_sections(&user_value);
        merge_non_null(user_value, &mut merged);

        let mut settings: Self =
            serde_json::from_value(merged).map_err(|error| SettingsError::Invalid {
                path: path.to_path_buf(),
                message: format!("设置结构错误：{error}"),
            })?;
        settings.unsupported = unsupported;
        settings.proxy = normalize_proxy(settings.proxy, path)?;
        // 传输也随之构造一次：代理 URL 的语法错误在**解析期**暴露，
        // 而不是等第一条命令发请求时才发现（坏代理会让所有 provider 一起失败）。
        let transport = build_transport(settings.proxy.as_deref()).map_err(|message| {
            SettingsError::Invalid {
                path: path.to_path_buf(),
                message,
            }
        })?;
        let _ = settings.transport.set(transport);

        Ok(settings)
    }

    /// 用户配置里出现、但还没有适配器的 section（例如 `deepseek`）。
    ///
    /// 不报错（同一份文件里可能还配着能用的 provider），但调用方应该提示：
    /// 写进这些 section 的端点**不会生效**；要连同一家请用
    /// `openai_compatible.<id>`（只要它兼容 OpenAI 协议）。
    pub fn unsupported_sections(&self) -> impl Iterator<Item = &str> {
        self.unsupported.iter().map(String::as_str)
    }

    /// 全进程共享的 HTTP 传输（一个连接池，按 `proxy` 首次使用时构造）。
    ///
    /// 代理语义在这里落定：显式设置 → `Proxy::explicit`；没写 → 跟随环境变量。
    /// 库层只接收这个已构造好的 `Arc`，不再自己读配置
    /// （见 aa-ai `docs/decisions/0010-transport-object.md`）。
    pub fn transport(&self) -> Result<Arc<dyn aa_http_client::HttpClient>, SettingsError> {
        if let Some(transport) = self.transport.get() {
            return Ok(transport.clone());
        }

        let transport = build_transport(self.proxy.as_deref())
            .map_err(|message| SettingsError::Transport { message })?;
        // 并发时可能另一个线程先 set；两边都是同一个配置，丢掉一个即可。
        let _ = self.transport.set(transport);
        Ok(self.transport.get().expect("刚刚 set 过").clone())
    }

    /// 用环境变量解析某个 provider 的凭证并构造客户端。
    pub fn resolve(&self, id: &str) -> Result<ResolvedProvider, SettingsError> {
        self.resolve_with(id, &EnvCredentials)
    }

    /// 凭证来源可替换（将来接系统钥匙串时不改配置结构）。
    ///
    /// 解析顺序由调用方决定；[`EnvCredentials`] 是当前唯一实现。
    pub fn resolve_with(
        &self,
        id: &str,
        credentials: &dyn Credentials,
    ) -> Result<ResolvedProvider, SettingsError> {
        // 与 Zed 一致：`openai_compatible` 条目优先于内置 section
        // （Zed 的内置 provider 先注册，compatible 后注册并覆盖同名 id）。
        if let Some(provider) = self.language_model_providers.openai_compatible.get(id) {
            return provider.resolve(id, &self.transport()?, credentials);
        }
        if id == LLAMA_CPP_PROVIDER_ID
            && let Some(provider) = &self.language_model_providers.llama_cpp
        {
            return provider.resolve(
                LLAMA_CPP_PROVIDER_ID,
                DEFAULT_LLAMA_CPP_BASE_URL,
                &self.transport()?,
                credentials,
            );
        }
        if id == OPENROUTER_PROVIDER_ID
            && let Some(provider) = &self.language_model_providers.openrouter
        {
            return provider.resolve(&self.transport()?, credentials);
        }
        if id == OLLAMA_PROVIDER_ID
            && let Some(provider) = &self.language_model_providers.ollama
        {
            // 原生 `/api/chat`，不是 OpenAI 兼容 `/v1`（那是另一个 section 的语义）。
            return provider.resolve(&self.transport()?, credentials);
        }

        Err(SettingsError::UnknownProvider {
            id: id.to_owned(),
            known: self.ids().map(str::to_owned).collect(),
        })
    }

    /// 已声明的 provider id（`openai_compatible` 的 key 已排序；内置 section 在后）。
    ///
    /// 被同名 `openai_compatible` 条目覆盖的内置 section 只出现一次。
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        let providers = &self.language_model_providers;
        let shadowed = |id: &str| providers.openai_compatible.contains_key(id);
        let compatible = providers.openai_compatible.keys().map(String::as_str);
        let llama_cpp = providers
            .llama_cpp
            .as_ref()
            .filter(|_| !shadowed(LLAMA_CPP_PROVIDER_ID))
            .map(|_| LLAMA_CPP_PROVIDER_ID);
        let openrouter = providers
            .openrouter
            .as_ref()
            .filter(|_| !shadowed(OPENROUTER_PROVIDER_ID))
            .map(|_| OPENROUTER_PROVIDER_ID);
        let ollama = providers
            .ollama
            .as_ref()
            .filter(|_| !shadowed(OLLAMA_PROVIDER_ID))
            .map(|_| OLLAMA_PROVIDER_ID);
        compatible.chain(llama_cpp).chain(openrouter).chain(ollama)
    }
}

/// 默认设置文件路径：`$AACODE_SETTINGS`，否则 `{config_dir}/aacode/settings.jsonc`。
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

fn parse_jsonc(
    text: &str,
    base_dir: &Path,
    path: &Path,
) -> Result<serde_json::Value, SettingsError> {
    let text = substitute::apply(text, base_dir);
    jsonc_parser::parse_to_serde_value(&text, &Default::default()).map_err(|error| {
        SettingsError::Invalid {
            path: path.to_path_buf(),
            message: format!("JSONC 解析失败：{error}"),
        }
    })
}

/// 规范化顶层 `proxy`：去空白、空串 = 没写、只接受 http/https。
///
/// 校验放在解析期而不是第一次发请求时：代理配错会让**所有**命令一起失败，
/// 越早报错越容易定位。
fn normalize_proxy(proxy: Option<String>, path: &Path) -> Result<Option<String>, SettingsError> {
    let Some(proxy) = proxy else {
        return Ok(None);
    };
    let proxy = proxy.trim();
    if proxy.is_empty() {
        return Ok(None);
    }

    if proxy.starts_with("socks") {
        return Err(SettingsError::Invalid {
            path: path.to_path_buf(),
            message: format!(
                "proxy `{proxy}` 是 socks 代理：需要 reqwest 的 `socks` feature，当前未启用"
            ),
        });
    }
    if !(proxy.starts_with("http://") || proxy.starts_with("https://")) {
        return Err(SettingsError::Invalid {
            path: path.to_path_buf(),
            message: format!("proxy `{proxy}` 需要以 http:// 或 https:// 开头"),
        });
    }

    Ok(Some(proxy.to_owned()))
}

/// 按顶层 `proxy` 构造共享传输（一个连接池）。
///
/// `None` = 跟随环境变量；`Some(url)` = 显式代理（URL 已由 [`normalize_proxy`]
/// 校过前缀，这里再由 `reqwest::Proxy::all` 做语法解析）。
fn build_transport(proxy: Option<&str>) -> Result<Arc<dyn aa_http_client::HttpClient>, String> {
    let proxy = match proxy {
        Some(url) => aa_http_client::Proxy::explicit(url),
        None => aa_http_client::Proxy::Environment,
    };
    aa_http_client::builder()
        .proxy(proxy)
        .build()
        .map_err(|error| error.to_string())
}

/// `language_model_providers` 下尚未实现的 section 名（只看用户写的那一层）。
fn unknown_sections(value: &serde_json::Value) -> BTreeSet<String> {
    const KNOWN: [&str; 4] = ["openai_compatible", "ollama", "llama.cpp", "openrouter"];

    value
        .get("language_model_providers")
        .and_then(serde_json::Value::as_object)
        .map(|object| {
            object
                .keys()
                .filter(|key| !KNOWN.contains(&key.as_str()))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// 把 `source` 深合并进 `target`，规则与 Zed 的 `merge_non_null_json_value_into` 一致：
///
/// - 两边都是对象 → 逐键递归；
/// - `source` 是 `null` → 保留 `target`（等于没写）；
/// - 其余（数组、标量）→ 整体覆盖。
fn merge_non_null(source: serde_json::Value, target: &mut serde_json::Value) {
    if let serde_json::Value::Object(source) = source {
        let target = match target {
            serde_json::Value::Object(target) => target,
            target => {
                *target = serde_json::Value::Object(Default::default());
                target.as_object_mut().expect("刚换成对象")
            }
        };

        for (key, value) in source {
            match target.get_mut(&key) {
                Some(existing) => merge_non_null(value, existing),
                // 目标里没有这个 key 时整棵子树插入；先把子树里的 null 去掉，
                // 这样 `"new-provider": { "headers": null }` 也等于「没写」。
                None => {
                    if let Some(value) = strip_nulls(value) {
                        target.insert(key, value);
                    }
                }
            }
        }
    } else if !source.is_null() {
        *target = strip_nulls(source).unwrap_or(serde_json::Value::Null);
    }
}

/// 递归去掉对象里的 `null` 值；全空的子树返回 `None`。
fn strip_nulls(value: serde_json::Value) -> Option<serde_json::Value> {
    match value {
        serde_json::Value::Object(object) => {
            let mut stripped = serde_json::Map::new();
            for (key, value) in object {
                if let Some(value) = strip_nulls(value) {
                    stripped.insert(key, value);
                }
            }
            Some(serde_json::Value::Object(stripped))
        }
        serde_json::Value::Null => None,
        other => Some(other),
    }
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

    fn parse_with_defaults(user: &str, defaults: &str) -> Settings {
        Settings::load_with_defaults(user, defaults, Path::new("/tmp/aacode/settings.jsonc"))
            .unwrap()
    }

    #[test]
    fn proxy_is_trimmed_and_empty_means_unset() {
        // 内置默认里的 `""`（用户没写）等价于「不设代理」。
        assert_eq!(parse_with_defaults("{}", r#"{"proxy": ""}"#).proxy, None);
        assert_eq!(parse_with_defaults("{}", "{}").proxy, None);

        let settings = parse_with_defaults(
            r#"{"proxy": "  http://127.0.0.1:8080  "}"#,
            r#"{"proxy": ""}"#,
        );
        assert_eq!(settings.proxy.as_deref(), Some("http://127.0.0.1:8080"));
    }

    #[test]
    fn proxy_requires_http_or_https() {
        let error = Settings::load_with_defaults(
            r#"{"proxy": "127.0.0.1:8080"}"#,
            "{}",
            Path::new("/tmp/aacode/settings.jsonc"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("http://"), "{error}");
    }

    #[test]
    fn socks_proxy_says_why_it_is_not_supported() {
        let error = Settings::load_with_defaults(
            r#"{"proxy": "socks5://127.0.0.1:1080"}"#,
            "{}",
            Path::new("/tmp/aacode/settings.jsonc"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("socks"), "{error}");
    }

    #[test]
    fn malformed_proxy_url_fails_at_parse_time() {
        // 前缀合法但 URL 本身不完整：reqwest 解析失败，应该在加载设置时就报错，
        // 不是等第一条命令发请求。这是应用层的设置校验，库层只报构造失败。
        let error = Settings::load_with_defaults(
            r#"{"proxy": "http://"}"#,
            "{}",
            Path::new("/tmp/aacode/settings.jsonc"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("代理"), "{error}");
    }

    #[test]
    fn resolve_carries_the_top_level_proxy() {
        // 只验证「代理存在时 resolve 仍然成功」：代理真正生效要发请求才看得见。
        let settings = parse_with_defaults(
            r#"{
                "proxy": "http://127.0.0.1:8080",
                "language_model_providers": {
                    "openai_compatible": {
                        "local": { "base_url": "http://localhost:9/v1", "auth": { "kind": "none" } }
                    }
                }
            }"#,
            "{}",
        );
        let credentials = FakeCredentials(HashMap::new());
        let resolved = settings.resolve_with("local", &credentials).unwrap();
        assert_eq!(resolved.base_url, "http://localhost:9/v1");
    }

    #[test]
    fn parses_jsonc_with_comments_and_trailing_commas() {
        let config = parse(
            r#"{
                // 注释和尾逗号都应该接受
                "language_model_providers": {
                    "openai_compatible": {
                        "agnes": {
                            "base_url": "https://apihub.agnes-ai.com/v1",
                        },
                    },
                },
            }"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["agnes"]);
    }

    #[test]
    fn null_provider_entry_keeps_default() {
        let defaults = r#"{"language_model_providers":{"openai_compatible":{"ollama":{
            "base_url":"http://localhost:11434/v1","auth":{"kind":"none"}
        }}}}"#;
        let user = r#"{"language_model_providers":{"openai_compatible":{"ollama":null}}}"#;

        let config = parse_with_defaults(user, defaults);
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["ollama"]);
        let resolved = config.resolve("ollama").unwrap();
        assert_eq!(resolved.base_url, "http://localhost:11434/v1");
    }

    #[test]
    fn null_provider_entry_without_default_is_absent() {
        let config = parse(r#"{"language_model_providers":{"openai_compatible":{"ollama":null}}}"#);
        assert_eq!(config.ids().count(), 0);
    }

    #[test]
    fn nested_null_in_new_provider_is_ignored() {
        let config = parse(
            r#"{"language_model_providers":{"openai_compatible":{"p":{
                "base_url":"https://example.com/v1",
                "auth":{"kind":"none"},
                "headers":null
            }}}}"#,
        );
        let provider = &config.language_model_providers.openai_compatible["p"];
        assert!(provider.headers.is_empty());
    }

    #[test]
    fn user_provider_deep_merges_into_default() {
        let defaults = r#"{"language_model_providers":{"openai_compatible":{"ollama":{
            "base_url":"http://localhost:11434/v1",
            "auth":{"kind":"none"},
            "headers":{"X-Default":"1"}
        }}}}"#;
        let user = r#"{"language_model_providers":{"openai_compatible":{"ollama":{
            "headers":{"X-User":"2"}
        }}}}"#;

        let config = parse_with_defaults(user, defaults);
        let provider = &config.language_model_providers.openai_compatible["ollama"];
        // 用户没写的字段保留默认值。
        assert_eq!(provider.base_url, "http://localhost:11434/v1");
        // 对象逐键合并。
        assert_eq!(provider.headers["X-Default"], "1");
        assert_eq!(provider.headers["X-User"], "2");
    }

    #[test]
    fn user_arrays_replace_defaults() {
        let defaults = r#"{"language_model_providers":{"openai_compatible":{"p":{
            "base_url":"https://example.com/v1","auth":{"kind":"none"},
            "available_models":[{"name":"a"},{"name":"b"}]
        }}}}"#;
        let user = r#"{"language_model_providers":{"openai_compatible":{"p":{
            "available_models":[{"name":"c"}]
        }}}}"#;

        let config = parse_with_defaults(user, defaults);
        let models = &config.language_model_providers.openai_compatible["p"].available_models;
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "c");
    }

    #[test]
    fn builtin_defaults_are_loaded() {
        let dir = std::env::temp_dir().join(format!(
            "aacode-defaults-test-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.jsonc");
        std::fs::write(&path, r#"{"language_model_providers":{"ollama":null}}"#).unwrap();

        let config = Settings::load(&path).unwrap();
        assert!(config.ids().any(|id| id == "ollama"));
        // 默认层的 ollama 是原生 section：resolve 得到的是 `/api/chat` 客户端，
        // 不是 OpenAI 兼容 `/v1`。
        let resolved = config.resolve("ollama").unwrap();
        assert_eq!(resolved.base_url, "http://localhost:11434");

        // 本地 llama.cpp 也是独立 section，默认无鉴权。
        assert!(config.ids().any(|id| id == "llama.cpp"));
        let llama = config.resolve("llama.cpp").unwrap();
        assert_eq!(llama.base_url, "http://localhost:8080");
        assert!(matches!(
            llama.client,
            crate::provider::ProviderClient::OpenAi(_)
        ));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn llama_cpp_section_is_independent_and_defaults_to_local() {
        let config = parse(r#"{"language_model_providers":{"llama.cpp":{}}}"#);
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["llama.cpp"]);
        // 已实现的 section，不该被提示「已忽略」。
        assert_eq!(config.unsupported_sections().count(), 0);

        let resolved = config
            .resolve_with("llama.cpp", &FakeCredentials(HashMap::new()))
            .unwrap();
        assert_eq!(resolved.id, "llama.cpp");
        assert_eq!(resolved.base_url, "http://localhost:8080");
        // wire 仍是 OpenAI 兼容：复用同一个客户端，只是 section 独立。
        assert!(matches!(
            resolved.client,
            crate::provider::ProviderClient::OpenAi(_)
        ));
    }

    #[test]
    fn llama_cpp_accepts_zed_api_url_alias() {
        let config = parse(
            r#"{"language_model_providers":{"llama.cpp":{"api_url":"http://localhost:9999/v1"}}}"#,
        );
        let resolved = config
            .resolve_with("llama.cpp", &FakeCredentials(HashMap::new()))
            .unwrap();
        assert_eq!(resolved.base_url, "http://localhost:9999/v1");
    }

    #[test]
    fn compatible_entry_shadows_llama_cpp_section() {
        let config = parse(
            r#"{"language_model_providers":{
                "openai_compatible":{
                    "llama.cpp":{"base_url":"http://proxy.example/v1","auth":{"kind":"none"}}
                },
                "llama.cpp":{"base_url":"http://localhost:8080"}
            }}"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["llama.cpp"]);
        assert_eq!(
            config.resolve("llama.cpp").unwrap().base_url,
            "http://proxy.example/v1"
        );
    }

    #[test]
    fn ollama_section_resolves_to_the_native_protocol() {
        let config = parse(
            r#"{"language_model_providers":{"ollama":{
                "base_url":"http://192.168.1.5:11434",
                "available_models":[{"name":"qwen3:8b","context_window":32768}]
            }}}"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["ollama"]);

        let provider = config
            .language_model_providers
            .ollama
            .as_ref()
            .expect("ollama section 应该被解析");
        assert_eq!(provider.base_url, "http://192.168.1.5:11434");
        assert_eq!(provider.available_models.len(), 1);

        let resolved = config.resolve("ollama").unwrap();
        assert_eq!(resolved.base_url, "http://192.168.1.5:11434");
        // 根地址：库层自己拼 `/api/chat`（不是 `.../v1`）。
        assert!(
            matches!(resolved.client, crate::provider::ProviderClient::Ollama(_)),
            "ollama section 应该用原生协议客户端"
        );
        // 模型表照旧生效。
        assert!(resolved.declares_model("qwen3:8b"));
        assert_eq!(resolved.model("qwen3:8b").context_window, Some(32768));
    }

    #[test]
    fn ollama_defaults_to_localhost() {
        let config = parse(r#"{"language_model_providers":{"ollama":{}}}"#);
        let provider = config
            .language_model_providers
            .ollama
            .as_ref()
            .expect("空对象也应该解析出默认值");
        assert_eq!(provider.base_url, "http://localhost:11434");
    }

    #[test]
    fn compatible_entry_shadows_ollama_section() {
        let config = parse(
            r#"{"language_model_providers":{
                "openai_compatible":{"ollama":{
                    "base_url":"http://localhost:11434/v1","auth":{"kind":"none"}
                }},
                "ollama":{"base_url":"http://localhost:11434"}
            }}"#,
        );
        // id 只出现一次，解析走 compatible 条目（与 Zed 的覆盖顺序一致）。
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["ollama"]);
        assert_eq!(
            config.resolve("ollama").unwrap().base_url,
            "http://localhost:11434/v1"
        );
    }

    #[test]
    fn openrouter_section_resolves_to_its_own_client() {
        let config = parse(
            r#"{"language_model_providers":{"openrouter":{
                "base_url":"https://openrouter.ai/api/v1",
                "app_url":"https://aacode.example",
                "app_title":"aacode",
                "headers":{"X-Title":"header-title"},
                "available_models":[{
                    "name":"google/gemma-4-31b-it:free",
                    "context_window":262144,
                    "capabilities":{"tools":true}
                }]
            }}}"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["openrouter"]);
        // openrouter 是已实现的 section，不该被提示「已忽略」。
        assert_eq!(config.unsupported_sections().count(), 0);

        let provider = config
            .language_model_providers
            .openrouter
            .as_ref()
            .expect("openrouter section 应该被解析");
        assert_eq!(provider.app_title.as_deref(), Some("aacode"));
        assert_eq!(provider.app_url.as_deref(), Some("https://aacode.example"));

        let credentials = FakeCredentials(HashMap::from([("OPENROUTER_API_KEY", "sk-or-test")]));
        let resolved = config.resolve_with("openrouter", &credentials).unwrap();
        assert_eq!(resolved.id, "openrouter");
        assert_eq!(resolved.base_url, "https://openrouter.ai/api/v1");
        assert!(
            matches!(
                resolved.client,
                crate::provider::ProviderClient::OpenRouter(_)
            ),
            "openrouter section 应该用独立的 OpenRouter 客户端"
        );
        assert_eq!(
            resolved.model("google/gemma-4-31b-it:free").context_window,
            Some(262_144)
        );
    }

    #[test]
    fn openrouter_defaults_to_official_base_url() {
        let config = parse(r#"{"language_model_providers":{"openrouter":{}}}"#);
        let provider = config
            .language_model_providers
            .openrouter
            .as_ref()
            .expect("空对象也应该解析出默认值");
        assert_eq!(provider.base_url, "https://openrouter.ai/api/v1");
        // 默认 Bearer + 由 section 名推导的变量名。
        let credentials = FakeCredentials(HashMap::from([("OPENROUTER_API_KEY", "sk-or-test")]));
        config.resolve_with("openrouter", &credentials).unwrap();
    }

    #[test]
    fn openrouter_app_name_is_an_alias_for_app_title() {
        let config =
            parse(r#"{"language_model_providers":{"openrouter":{"app_name":"alias-title"}}}"#);
        let provider = config.language_model_providers.openrouter.as_ref().unwrap();
        assert_eq!(provider.app_title.as_deref(), Some("alias-title"));
    }

    #[test]
    fn compatible_entry_shadows_openrouter_section() {
        let config = parse(
            r#"{"language_model_providers":{
                "openai_compatible":{"openrouter":{
                    "base_url":"https://proxy.example/v1","auth":{"kind":"none"}
                }},
                "openrouter":{"base_url":"https://openrouter.ai/api/v1"}
            }}"#,
        );
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["openrouter"]);
        let resolved = config.resolve("openrouter").unwrap();
        assert_eq!(resolved.base_url, "https://proxy.example/v1");
        // 同名 compatible 条目优先：拿的是 OpenAI 兼容客户端。
        assert!(matches!(
            resolved.client,
            crate::provider::ProviderClient::OpenAi(_)
        ));
    }

    #[test]
    fn openrouter_missing_credential_lists_default_name() {
        let config = parse(r#"{"language_model_providers":{"openrouter":{}}}"#);
        let error = config
            .resolve_with("openrouter", &FakeCredentials(HashMap::new()))
            .unwrap_err()
            .to_string();
        assert!(error.contains("OPENROUTER_API_KEY"), "{error}");
    }

    #[test]
    fn unsupported_sections_are_collected_not_fatal() {
        let config = parse(
            r#"{"language_model_providers":{
                "anthropic":{"base_url":"https://example.com"},
                "openai_compatible":{"p":{"base_url":"https://example.com/v1"}}
            }}"#,
        );
        assert_eq!(
            config.unsupported_sections().collect::<Vec<_>>(),
            vec!["anthropic"]
        );
        // 已实现的 section 照常工作。
        assert_eq!(config.ids().collect::<Vec<_>>(), vec!["p"]);
    }

    #[test]
    fn default_placeholders_are_not_reported_as_unsupported() {
        // 内置默认文件里的 `deepseek` 等占位不该让每次启动都报警。
        let defaults = r#"{"language_model_providers":{"deepseek":{"base_url":"https://api.deepseek.com/v1"}}}"#;
        let config =
            parse_with_defaults(r#"{"language_model_providers":{"ollama":null}}"#, defaults);
        assert_eq!(config.unsupported_sections().count(), 0);
    }

    #[test]
    fn missing_credential_lists_tried_names() {
        let config = parse(
            r#"{"language_model_providers":{"openai_compatible":{"minekuai":{
                "base_url":"https://minekuai.cn/v1",
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
            r#"{"language_model_providers":{"openai_compatible":{"agnes":{
                "base_url":"https://example.com/v1",
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
            r#"{"language_model_providers":{"openai_compatible":{"ollama":{
                "base_url":"http://localhost:11434/v1","auth":{"kind":"none"}
            }}}}"#,
        );
        config
            .resolve_with("ollama", &FakeCredentials(HashMap::new()))
            .unwrap();
    }

    #[test]
    fn model_facts_and_capabilities_are_applied() {
        let config = parse(
            r#"{"language_model_providers":{"openai_compatible":{"p":{
                "base_url":"https://example.com/v1",
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
            r#"{"language_model_providers":{"openai_compatible":{"p":{
                "base_url":"https://example.com/v1","auth":{"kind":"none"},
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
            r#"{"language_model_providers":{"openai_compatible":{"p":{
                "base_url":"https://example.com/v1","auth":{"kind":"none"},
                "available_models":[{"name":"m"},{"name":"m"}]
            }}}}"#,
        );
        let error = config.resolve("p").unwrap_err().to_string();
        assert!(error.contains("`m`") && error.contains("重复"), "{error}");
    }

    #[test]
    fn responses_only_models_are_rejected_for_now() {
        let config = parse(
            r#"{"language_model_providers":{"openai_compatible":{"p":{
                "base_url":"https://example.com/v1","auth":{"kind":"none"},
                "available_models":[{"name":"m","capabilities":{"chat_completions":false}}]
            }}}}"#,
        );
        let error = config.resolve("p").unwrap_err().to_string();
        assert!(error.contains("chat_completions"), "{error}");
    }

    #[test]
    fn unknown_provider_lists_known_ids() {
        let config = parse(
            r#"{"language_model_providers":{"openai_compatible":{"agnes":{"base_url":"https://example.com/v1"}}}}"#,
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
                "language_model_providers": {{
                    "openai_compatible": {{
                        "p": {{
                            "base_url": "https://example.com/v1",
                            "auth": {{"kind":"bearer","env":["FROM_FILE"]}},
                            "headers": {{"X-From-File":"{{file:{} }}"}}
                        }}
                    }}
                }}
            }}"#,
            key_file.display()
        );
        let config = Settings::parse(&text, &dir.join("settings.jsonc")).unwrap();
        let headers = &config.language_model_providers.openai_compatible["p"].headers;
        assert_eq!(headers["X-From-File"], "sk-from-file");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn env_substitution_escapes_quotes() {
        let text = r#"{"language_model_providers":{"openai_compatible":{"p":{
            "base_url":"https://example.com/v1",
            "headers":{"X-Quote":"{env:AACODE_TEST_QUOTE}"}}}}}"#;
        // 不依赖真实进程 env：这里只验证「缺失时替换成空串」，解析仍然成立。
        let config = Settings::parse(text, Path::new("/tmp/p.jsonc")).unwrap();
        assert_eq!(
            config.language_model_providers.openai_compatible["p"].headers["X-Quote"],
            ""
        );
    }
}
