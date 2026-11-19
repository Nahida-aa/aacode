//! 设置层的错误。全部是「哪里错了 + 怎么修」的形状，CLI 直接打印。

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("读取设置文件 {path} 失败：{source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("设置文件 {path} 无效：{message}")]
    Invalid { path: PathBuf, message: String },

    #[error("没有 provider `{id}`；已声明：{known:?}")]
    UnknownProvider { id: String, known: Vec<String> },

    #[error("provider `{provider}` 设置有问题：{message}")]
    Provider { provider: String, message: String },

    #[error("provider `{provider}` 里模型 `{model}` 重复声明")]
    DuplicateModel { provider: String, model: String },

    #[error("provider `{provider}` 的模型 `{model}` 暂不支持：{reason}")]
    UnsupportedModel {
        provider: String,
        model: String,
        reason: String,
    },

    #[error("provider `{provider}` 缺少凭证：环境变量 {names:?} 都没有值")]
    MissingCredential {
        provider: String,
        names: Vec<String>,
    },
}
