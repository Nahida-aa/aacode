//! 凭证来源。
//!
//! 配置只存**变量名**，不存密钥。当前实现读环境变量；系统钥匙串 / OAuth 刷新
//! 等实现放在应用里，实现同一个 trait 即可（`provider` 参数就是给按 id 索引的
//! 凭证库用的）。

/// 按 provider + 变量名取一个非空凭证。
pub trait Credentials: Send + Sync {
    /// `provider` 是配置里的 id（`agnes` / `minekuai` / ...）。
    fn get(&self, provider: &str, name: &str) -> Option<String>;
}

/// 环境变量实现：名字存在且非空才算命中。
#[derive(Debug, Clone, Copy, Default)]
pub struct EnvCredentials;

impl Credentials for EnvCredentials {
    fn get(&self, _provider: &str, name: &str) -> Option<String> {
        match std::env::var(name) {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        }
    }
}
