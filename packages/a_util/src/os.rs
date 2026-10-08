use anyhow::Result;
use std::path::PathBuf;

use gpui_util::ResultExt as _;

/// Prevents execution of the application with root privileges on Unix systems.
///
/// This function checks if the current process is running with root privileges
/// and terminates the program with an error message unless explicitly allowed via the
/// `ZED_ALLOW_ROOT` environment variable.
#[cfg(unix)]
pub fn prevent_root_execution() {
    let is_root = nix::unistd::geteuid().is_root();
    let allow_root = std::env::var("ZED_ALLOW_ROOT").is_ok_and(|val| val == "true");

    if is_root && !allow_root {
        eprintln!(
            "\
Error: Running Zed as root or via sudo is unsupported.
       Doing so (even once) may subtly break things for all subsequent non-root usage of Zed.
       It is untested and not recommended, don't complain when things break.
       If you wish to proceed anyways, set `ZED_ALLOW_ROOT=true` in your environment."
        );
        std::process::exit(1);
    }
}

/// Returns a shell escaped path for the current zed executable
#[cfg(not(target_family = "wasm"))]
pub fn get_shell_safe_zed_path(shell_kind: util::shell::ShellKind) -> anyhow::Result<String> {
    use util::paths::PathExt;
    use anyhow::Context as _;
    let mut zed_path =
        std::env::current_exe().context("Failed to determine current zed executable path.")?;
    if cfg!(target_os = "linux")
        && !zed_path.is_file()
        && let Some(truncated) = zed_path
            .clone()
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|n| n.strip_suffix(" (deleted)"))
    {
        // Might have been deleted during update; let's use the new binary if there is one.
        zed_path.set_file_name(truncated);
    }

    zed_path
        .try_shell_safe(shell_kind)
        .context("Failed to shell-escape Zed executable path.")
}

/// Returns a path for the zed cli executable, this function
/// should be called from the zed executable, not zed-cli.
pub fn get_zed_cli_path() -> Result<PathBuf> {
    use anyhow::Context as _;
    let zed_path =
        std::env::current_exe().context("Failed to determine current zed executable path.")?;
    let parent = zed_path
        .parent()
        .context("Failed to determine parent directory of zed executable path.")?;

    let possible_locations: &[&str] = if cfg!(target_os = "macos") {
        // On macOS, the zed executable and zed-cli are inside the app bundle,
        // so here ./cli is for both installed and development builds.
        &["./cli"]
    } else if cfg!(target_os = "windows") {
        // bin/zed.exe is for installed builds, ./cli.exe is for development builds.
        &["bin/zed.exe", "./cli.exe"]
    } else if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
        // bin is the standard, ./cli is for the target directory in development builds.
        &["../bin/zed", "./cli"]
    } else {
        anyhow::bail!("unsupported platform for determining zed-cli path");
    };

    possible_locations
        .iter()
        .find_map(|p| {
            parent
                .join(p)
                .canonicalize()
                .ok()
                .filter(|p| p != &zed_path)
        })
        .with_context(|| {
            format!(
                "could not find zed-cli from any of: {}",
                possible_locations.join(", ")
            )
        })
}

#[cfg(unix)]
fn load_shell_from_passwd() -> Result<()> {
    let buflen = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) } {
        n if n < 0 => 1024,
        n => n as usize,
    };
    let mut buffer = Vec::with_capacity(buflen);

    let mut pwd: std::mem::MaybeUninit<libc::passwd> = std::mem::MaybeUninit::uninit();
    let mut result: *mut libc::passwd = std::ptr::null_mut();

    let uid = unsafe { libc::getuid() };
    let status = unsafe {
        libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_char,
            buflen,
            &mut result,
        )
    };
    anyhow::ensure!(!result.is_null(), "passwd entry for uid {} not found", uid);

    // SAFETY: If `getpwuid_r` doesn't error, we have the entry here.
    let entry = unsafe { pwd.assume_init() };

    anyhow::ensure!(
        status == 0,
        "call to getpwuid_r failed. uid: {}, status: {}",
        uid,
        status
    );
    anyhow::ensure!(
        entry.pw_uid == uid,
        "passwd entry has different uid ({}) than getuid ({}) returned",
        entry.pw_uid,
        uid,
    );

    let shell = unsafe { std::ffi::CStr::from_ptr(entry.pw_shell).to_str().unwrap() };
    let should_set_shell = std::env::var("SHELL").map_or(true, |shell_env| {
        shell_env != shell && !std::path::Path::new(&shell_env).exists()
    });

    if should_set_shell {
        log::info!(
            "updating SHELL environment variable to value from passwd entry: {:?}",
            shell,
        );
        unsafe { std::env::set_var("SHELL", shell) };
    }

    Ok(())
}

#[cfg(unix)]
pub async fn load_login_shell_environment() -> Result<()> {
    use anyhow::Context as _;

    load_shell_from_passwd().log_err();

    // If possible, we want to `cd` in the user's `$HOME` to trigger programs
    // such as direnv, asdf, mise, ... to adjust the PATH. These tools often hook
    // into shell's `cd` command (and hooks) to manipulate env.
    // We do this so that we get the env a user would have when spawning a shell
    // in home directory.
    for (name, value) in
        crate::shell_env::capture(util::get_system_shell(), &[], util::paths::home_dir())
            .await
            .with_context(|| {
                format!("capturing environment with {:?}", util::get_system_shell())
            })?
    {
        // Skip SHLVL to prevent it from polluting Zed's process environment.
        // The login shell used for env capture increments SHLVL, and if we propagate it,
        // terminals spawned by Zed will inherit it and increment again, causing SHLVL
        // to start at 2 instead of 1 (and increase by 2 on each reload).
        if name == "SHLVL" {
            continue;
        }
        unsafe { std::env::set_var(&name, &value) };
    }

    log::info!(
        "set environment variables from shell:{}, path:{}",
        std::env::var("SHELL").unwrap_or_default(),
        std::env::var("PATH").unwrap_or_default(),
    );

    Ok(())
}
