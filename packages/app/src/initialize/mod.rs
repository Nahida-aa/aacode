//! observe_new 注册 — 对齐 Zed `crates/zed/src/zed.rs::initialize_workspace`。
//!
//! 关键 insight：Sidebar / TitleBar / Panels 都不在 open_window 里手动创建。
//! 而是在全局 init 之后，通过 `cx.observe_new` 注册回调。
//! 一旦 MultiWorkspace / Workspace entity 被 new，回调自动触发并注入依赖。
//!
//! 之所以要在 `cx.defer` 里创建，是因为 MultiWorkspace::new / Workspace::new 的订阅
//! 链需要先建立好，再注入子 entity（Sidebar 等）。

use gpui::{App, AppContext, Context, KeyBinding, PromptLevel, Window, TaskExt as _};
use gpui_util::ResultExt as _;
use util::asset_str;
use workspace::with_active_or_new_workspace;
use std::borrow::Cow;
use std::path::Path;

use language::Capability;
use settings::Settings as _;
use vim_mode_setting::HelixModeSetting;
use workspace::{MultiWorkspace, Workspace};

pub mod panels;
pub mod panes;
pub mod workspace_init;

pub use workspace_init::initialize_workspace;

/// 注册 App 级 action handler —— 对齐 Zed `crates/zed/src/zed.rs::init`（L193-L334）。
///
/// Zed 把这些 handler 放在 `zed.rs::init` 里，由 `main.rs` 在 crate init 链之后调用。
/// aacode 原先完全没有这一层（`core::init` 是空壳且从未被调用），导致菜单里绑的
/// `Quit` / `About` / `OpenLicenses` 等 action 点了没反应。
///
/// 与 `initialize_workspace` 的区别：后者注册的是 `observe_new` 回调（workspace 创建
/// 时注入 UI），本函数注册的是 **App 全局** action handler，不依赖 workspace 是否已存在
/// （需要 workspace 的会内部走 `with_active_or_new_workspace`）。
pub fn init(cx: &mut App) {
    cx.on_action(|_: &crate::app_menus::Hide, cx| cx.hide());
    #[cfg(target_os = "macos")]
    cx.on_action(|_: &crate::app_menus::HideOthers, cx| cx.hide_other_apps());
    #[cfg(target_os = "macos")]
    cx.on_action(|_: &crate::app_menus::ShowAll, cx| cx.unhide_other_apps());
    cx.on_action(quit);

    cx.on_action(|_: &aacode_actions::OpenTelemetryLog, cx| {
        cx.reveal_path(paths::log_file().as_path());
    });

    cx.on_action(|_: &aacode_actions::OpenSettingsFile, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::settings_file(),
                || settings::initial_user_settings_content().as_ref().into(),
                window,
                cx,
            );
        });
    })
    .on_action(|_: &aacode_actions::OpenKeymapFile, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::keymap_file(),
                || settings::initial_keymap_content().as_ref().into(),
                window,
                cx,
            );
        });
    })
    .on_action(|_: &aacode_actions::OpenProjectTasks, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::tasks_file(),
                || settings::initial_tasks_content().as_ref().into(),
                window,
                cx,
            );
        });
    })
    .on_action(|_: &aacode_actions::OpenProjectDebugTasks, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::debug_scenarios_file(),
                || settings::initial_debug_tasks_content().as_ref().into(),
                window,
                cx,
            );
        });
    });

    cx.on_action(|_: &aacode_actions::OpenLicenses, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                asset_str::<aa_gpui_kit_assets::Assets>("licenses.md"),
                "Open Source License Attribution",
                "Markdown",
                window,
                cx,
            );
        });
    })
    .on_action(|_: &aacode_actions::OpenDefaultKeymap, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                settings::default_keymap(),
                "Default Key Bindings",
                "JSON",
                window,
                cx,
            );
        });
    })
    .on_action(|_: &crate::app_menus::OpenDefaultSettings, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                settings::default_settings(),
                "Default Settings",
                "JSON",
                window,
                cx,
            );
        });
    })
    .on_action(|_: &crate::app_menus::ShowDefaultSemanticTokenRules, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                settings::default_semantic_token_rules(),
                "Default Semantic Token Rules",
                "JSONC",
                window,
                cx,
            );
        });
    });

    cx.on_action(|_: &aacode_actions::About, cx| {
        crate::core::open_about_window(cx);
    });

    // 安装 CLI 工具到系统 PATH（对齐 Zed `zed.rs::init` 里的
    // `cx.on_action(|_: &install_cli::InstallCliBinary, cx| install_cli_binary(cx))`）。
    // `install_cli_binary` 需要 Window + Workspace 上下文，故走 `with_active_or_new_workspace`。
    #[cfg(not(target_os = "windows"))]
    cx.on_action(|_: &install_cli::InstallCliBinary, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            install_cli::install_cli_binary(window, cx);
        });
    });

    // project_symbols 的 handler 通过 `observe_new(Workspace)` 注册，
    // 需在 App 级 init 调用一次（对齐 Zed `zed.rs::init`）。
    project_symbols::init(cx);
}

/// 对齐 Zed `crates/zed/src/zed.rs::quit`（L1753）。
///
/// 与 `workspace::core::lifecycle::reload` 是同一套模板：按 `confirm_quit`
/// 设置弹确认框 → `prepare_windows_to_quit` 处理未保存文件 → 真正退出。
fn quit(_: &aacode_actions::Quit, cx: &mut App) {
    let should_confirm = workspace::WorkspaceSettings::get_global(cx).confirm_quit;
    let mut workspace_windows = cx
        .windows()
        .into_iter()
        .filter_map(|window| window.downcast::<MultiWorkspace>())
        .collect::<Vec<_>>();

    // 多个窗口都有未保存改动时，先在当前激活窗口弹提示，避免跳窗口。
    workspace_windows.sort_by_key(|window| window.is_active(cx) == Some(false));

    let mut prompt = None;
    if let (true, Some(window)) = (should_confirm, workspace_windows.first()) {
        prompt = window
            .update(cx, |_, window, cx| {
                window.prompt(
                    PromptLevel::Info,
                    "Are you sure you want to quit?",
                    None,
                    &["Quit", "Cancel"],
                    cx,
                )
            })
            .ok();
    }

    cx.spawn(async move |cx| {
        if let Some(prompt) = prompt {
            let answer = prompt.await?;
            if answer != 0 {
                return anyhow::Ok(());
            }
        }

        if !workspace::workspace::core::lifecycle::prepare_windows_to_quit(&workspace_windows, cx).await {
            return anyhow::Ok(());
        }
        cx.update(|cx| cx.quit());
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

/// 对齐 Zed `crates/zed/src/zed.rs::open_settings_file`（L2723）。
///
/// 为 settings 单独建 worktree，避免每次开合都重启 LSP。
fn open_settings_file(
    abs_path: &'static Path,
    default_content: impl 'static + Send + FnOnce() -> rope::Rope,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.spawn_in(window, async move |workspace, cx| {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.with_local_or_wsl_workspace(window, cx, move |workspace, window, cx| {
                    let project = workspace.project().clone();

                    cx.spawn_in(window, async move |workspace, cx| {
                        // 只为建 worktree（避免每次开合 settings 都重启 LSP），
                        // 句柄本身不需要持有。
                        let (_worktree, _) = project
                            .update(cx, |project, cx| {
                                project.find_or_create_worktree(paths::config_dir(), false, cx)
                            })
                            .await?;

                        workspace
                            .update_in(cx, |_, window, cx| {
                                workspace::workspace::open::file::create_and_open_local_file(
                                    abs_path,
                                    window,
                                    cx,
                                    default_content,
                                )
                            })?
                            .await?;

                        anyhow::Ok(())
                    })
                })
            })?
            .await
    })
    .detach_and_log_err(cx);
}

/// 对齐 Zed `crates/zed/src/zed.rs::open_bundled_file`（L2655）。
///
/// 打开一个只读的内嵌文本（licenses / 默认 keymap 等）；已打开则直接激活。
fn open_bundled_file(
    workspace: &mut Workspace,
    text: Cow<'static, str>,
    title: &'static str,
    language: &'static str,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let existing = workspace
        .items_of_type::<editor::Editor>(cx)
        .find(|editor| {
            editor.read_with(cx, |editor, cx| {
                editor.read_only(cx)
                    && editor.title(cx).as_ref() == title
                    && editor
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .is_some_and(|buffer| buffer.read(cx).file().is_none())
            })
        });
    if let Some(existing) = existing {
        workspace.activate_item(&existing, true, true, window, cx);
        return;
    }

    let language = workspace.app_state().languages.language_for_name(language);
    cx.spawn_in(window, async move |workspace, cx| {
        let language = language.await.log_err();
        workspace
            .update_in(cx, move |workspace, window, cx| {
                let project = workspace.project().clone();
                let buffer = project.update(cx, move |project, cx| {
                    project.create_buffer(language, false, cx)
                });
                cx.spawn_in(window, async move |workspace, cx| {
                    let buffer = buffer.await?;
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_text(text.into_owned(), cx);
                        buffer.set_capability(Capability::ReadOnly, cx);
                    });
                    let buffer = cx.new(|cx| {
                        multi_buffer::MultiBuffer::singleton(buffer, cx).with_title(title.into())
                    });
                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(cx.new(|cx| {
                                let mut editor = editor::Editor::for_multibuffer(
                                    buffer,
                                    Some(project.clone()),
                                    window,
                                    cx,
                                );
                                editor.set_read_only(true);
                                editor.set_should_serialize(false, cx);
                                editor.set_breadcrumb_header(title.into());
                                editor
                            })),
                            None,
                            true,
                            window,
                            cx,
                        )
                    })
                })
            })?
            .await
    })
    .detach_and_log_err(cx);
}

/// 把内置默认快捷键（`assets/keymaps/default-{linux,macos,windows}.json`）绑定到全局
/// `gpui::Keymap`。对齐 Zed `crates/zed/src/zed.rs::load_default_keymap`（L2347）。
///
/// 没有这一步，应用里就**一个默认快捷键都没有**：编辑器内建面板
/// （`cx.bind_keys({...})`）之外的绑定——删除字符、全选、保存、查找——全部静默失效，
/// 且不报任何错。表现为「输入 backspace / ctrl-a 没反应」，同时 KeymapEditor 面板
/// 只有 action 列有内容（action 来自代码注册，绑定列要 keymap 数据才有）。
///
/// 加载顺序与 Zed 一致：default → base_keymap（如 vscode 方案）→ vim 模式。
/// 后者覆盖前者，用户 keymap 最后加载再覆盖它们。
///
/// 调用点见 `main.rs`：必须在 `settings::init(cx)` 之后（要读 `BaseKeymap` 全局）。
pub fn load_default_keymap(cx: &mut App) {
    let base_keymap = *settings::BaseKeymap::get_global(cx);
    if base_keymap == settings::BaseKeymap::None {
        return;
    }

    let default = settings::KeymapFile::load_asset_partial(
        settings::DEFAULT_KEYMAP_PATH,
        Some(settings::KeybindSource::Default),
        cx,
    )
    .unwrap_or_else(|e| {
        panic!("failed to load built-in keymap {}: {e}", settings::DEFAULT_KEYMAP_PATH)
    });
    cx.bind_keys(filter_disabled_ai_bindings(default, cx));

    if let Some(asset_path) = base_keymap.asset_path() {
        let bindings = settings::KeymapFile::load_asset_partial(
            asset_path,
            Some(settings::KeybindSource::Base),
            cx,
        )
        .unwrap_or_else(|e| panic!("failed to load base keymap {asset_path}: {e}"));
        cx.bind_keys(filter_disabled_ai_bindings(bindings, cx));
    }

    if HelixModeSetting::get_global(cx).0 {
        let vim = settings::KeymapFile::load_asset_partial(
            settings::VIM_KEYMAP_PATH,
            Some(settings::KeybindSource::Vim),
            cx,
        )
        .unwrap_or_else(|e| {
            panic!("failed to load built-in keymap {}: {e}", settings::VIM_KEYMAP_PATH)
        });
        cx.bind_keys(filter_disabled_ai_bindings(vim, cx));
    }
}

/// 用户开了 `disable_ai` 时，剥掉 AI 相关的绑定，避免按键被一个「handler 静默
/// no-op」的 action 抢占（`ctrl-enter` 之类就无法正常触发）。对齐 Zed
/// `crates/zed/src/zed.rs::filter_disabled_ai_bindings`（L2403）。
fn filter_disabled_ai_bindings(bindings: Vec<KeyBinding>, cx: &App) -> Vec<KeyBinding> {
    if !project::DisableAiSettings::get_global(cx).disable_ai {
        return bindings;
    }
    bindings
        .into_iter()
        .filter(|binding| !is_ai_keybinding(binding))
        .collect()
}

fn is_ai_keybinding(binding: &KeyBinding) -> bool {
    /// 见 Zed `crates/zed/src/zed.rs::AI_ACTION_NAMESPACES`（L2387）。
    const AI_ACTION_NAMESPACES: &[&str] = &[
        "acp::",
        "agent::",
        "assistant::",
        "edit_prediction::",
        "inline_assistant::",
        "zeta::",
    ];
    let name = binding.action().name();
    AI_ACTION_NAMESPACES
        .iter()
        .any(|namespace| name.starts_with(namespace))
}
