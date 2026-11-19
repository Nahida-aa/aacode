//! `{env:VAR}` / `{file:path}` 文本替换。
//!
//! 在 JSONC 解析**之前**替换，和 opencode 的 `config/variable.ts` 同款：
//! 值总按 JSON 字符串内容转义，所以占位符要写在引号里：
//!
//! ```jsonc
//! "auth": { "kind": "bearer", "env": ["AGNES_API_KEY"] },
//! "headers": { "X-Title": "{env:AACODE_TITLE}" }
//! ```
//!
//! - `{env:VAR}` 缺失 → 空串（真正缺凭证会在 resolve 时报出变量名）；
//! - `{file:path}` 缺失/不可读 → 空串；`~/` 展开到 home；相对路径相对配置文件；
//! - 行注释里的占位符原样保留，不触发替换。

use std::path::{Path, PathBuf};

pub(crate) fn apply(text: &str, base_dir: &Path) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;

    while let Some(directive) = next_directive(text, cursor) {
        // 注释里的 `{env:...}` 是给人看的，不要读进配置。
        if in_line_comment(text, directive.start) {
            out.push_str(&text[cursor..directive.end]);
            cursor = directive.end;
            continue;
        }

        let value = match directive.kind {
            Kind::Env => std::env::var(directive.arg).unwrap_or_default(),
            Kind::File => read_file(directive.arg, base_dir),
        };

        out.push_str(&text[cursor..directive.start]);
        out.push_str(&escape_json(&value));
        cursor = directive.end;
    }

    out.push_str(&text[cursor..]);
    out
}

#[derive(Clone, Copy)]
enum Kind {
    Env,
    File,
}

struct Directive<'a> {
    start: usize,
    end: usize,
    kind: Kind,
    arg: &'a str,
}

fn next_directive(text: &str, from: usize) -> Option<Directive<'_>> {
    let env = text[from..]
        .find("{env:")
        .map(|index| (from + index, Kind::Env));
    let file = text[from..]
        .find("{file:")
        .map(|index| (from + index, Kind::File));

    let (start, kind) = match (env, file) {
        (Some(env), Some(file)) if env.0 <= file.0 => env,
        (Some(env), None) => env,
        (None, Some(file)) => file,
        (Some(_), Some(file)) => file,
        (None, None) => return None,
    };

    let arg_start = start
        + match kind {
            Kind::Env => "{env:".len(),
            Kind::File => "{file:".len(),
        };
    let arg_len = text[arg_start..].find('}')?;
    let end = arg_start + arg_len + 1;

    Some(Directive {
        start,
        end,
        kind,
        arg: &text[arg_start..arg_start + arg_len],
    })
}

fn in_line_comment(text: &str, index: usize) -> bool {
    let line_start = text[..index].rfind('\n').map_or(0, |newline| newline + 1);
    text[line_start..index].trim_start().starts_with("//")
}

fn read_file(raw: &str, base_dir: &Path) -> String {
    let raw = raw.trim();
    let path = match raw.strip_prefix("~/") {
        Some(rest) => dirs::home_dir().unwrap_or_default().join(rest),
        None => {
            let path = PathBuf::from(raw);
            if path.is_absolute() {
                path
            } else {
                base_dir.join(path)
            }
        }
    };

    // 一个过期的 secret 文件引用不该让整份配置解析失败；缺凭证会在 resolve 时
    // 报出明确的变量名。
    std::fs::read_to_string(path)
        .map(|content| content.trim().to_owned())
        .unwrap_or_default()
}

fn escape_json(value: &str) -> String {
    let quoted = serde_json::to_string(value).expect("字符串序列化不会失败");
    quoted[1..quoted.len() - 1].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_env_becomes_empty_string() {
        let text = r#"{"title":"{env:AACODE_TEST_DEFINITELY_UNSET}"}"#;
        assert_eq!(apply(text, Path::new("/tmp")), r#"{"title":""}"#);
    }

    #[test]
    fn file_reference_is_relative_and_trimmed() {
        let dir = std::env::temp_dir().join(format!("aacode-substitute-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("secret.txt"), "  value \n").unwrap();

        let text = r#"{"key":"{file:secret.txt}"}"#;
        assert_eq!(apply(text, &dir), r#"{"key":"value"}"#);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn placeholders_in_line_comments_are_kept() {
        let text = "{\n // 用 {env:FOO} 替换\n \"a\": 1\n}";
        assert_eq!(apply(text, Path::new("/tmp")), text);
    }

    #[test]
    fn values_are_json_escaped() {
        assert_eq!(escape_json("a\"b\\c"), "a\\\"b\\\\c");
    }
}
