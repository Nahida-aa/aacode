//! 配置层的错误。全部是「哪里错了 + 怎么修」的形状，CLI 直接打印。

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("读取配置 {path} 失败：{source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("配置 {path} 无效：{message}")]
    Invalid { path: PathBuf, message: String },

    #[error("没有 provider `{id}`；已配置：{known:?}")]
    UnknownProvider { id: String, known: Vec<String> },

    #[error("provider `{provider}` 配置有问题：{message}")]
    Provider { provider: String, message: String },

    #[error("provider `{provider}` 缺少凭证：环境变量 {names:?} 都没有值")]
    MissingCredential {
        provider: String,
        names: Vec<String>,
    },
}
