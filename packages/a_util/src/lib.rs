mod os;
pub use os::{
    get_shell_safe_zed_path, get_zed_cli_path,
    load_login_shell_environment, prevent_root_execution,
};

pub mod shell_env;
