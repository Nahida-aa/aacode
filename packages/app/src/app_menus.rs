//! 应用菜单（对齐 Zed `crates/zed/src/zed/app_menus.rs`）。
//!
//! Zed 原版的 app 级 action（`Hide` / `HideOthers` / `ShowAll` / `Minimize` /
//! `Zoom` / `ToggleFullScreen`）定义在 `crates/zed/src/zed.rs:121` 的
//! `actions!(zed, [...])` 块里。aacode 没有 `zed.rs` 这个等价文件（入口逻辑拆在
//! `main.rs` + `initialize/mod.rs`），所以这些 action 定义在本文件顶部。
//!
//! Handler 分两类，落点与 Zed 一致：
//! - App 级（`Hide` / `HideOthers` / `ShowAll`）→ 本文件 [`init`] 用 `cx.on_action`。
//! - Window 级（`Minimize` / `Zoom` / `ToggleFullScreen`）→ `initialize/mod.rs` 里
//!   用 `workspace.register_action`，因为需要 `&mut Window`。

use gpui::{App, Menu, MenuItem, NoAction, OsAction, actions};
use settings::Settings;

// Zed 把这些 app 级 action 定义在 `crates/zed/src/zed.rs:121` 的 `actions!(zed, [...])`
// 里；aacode 没有 zed.rs 等价文件，集中定义在此。
// handler 注册见 `initialize::init`（对齐 Zed `zed.rs::init`）。
actions!(
    app,
    [
        /// Hides the application window.
        Hide,
        /// Hides all other application windows.
        HideOthers,
        /// Minimizes the current window.
        Minimize,
        /// Opens the default settings file.
        OpenDefaultSettings,
        /// Shows all hidden windows.
        ShowAll,
        /// Shows the default semantic token rules (read-only).
        ShowDefaultSemanticTokenRules,
        /// Toggles fullscreen mode.
        ToggleFullScreen,
        /// Zooms the window.
        Zoom,
    ]
);

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    let mut view_items = vec![
        MenuItem::action(
            "Zoom In",
            aacode_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Zoom Out",
            aacode_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Reset Zoom",
            aacode_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Reset All Zoom",
            aacode_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action("Toggle Left Dock", workspace::ToggleLeftDock),
        MenuItem::action("Toggle Right Dock", workspace::ToggleRightDock),
        MenuItem::action("Toggle Bottom Dock", workspace::ToggleBottomDock),
        MenuItem::action("Toggle All Docks", workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: "Editor Layout".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Split Up", workspace::SplitUp::default()),
                MenuItem::action("Split Down", workspace::SplitDown::default()),
                MenuItem::action("Split Left", workspace::SplitLeft::default()),
                MenuItem::action("Split Right", workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action("Project Panel", aacode_actions::project_panel::ToggleFocus),
        MenuItem::action("Outline Panel", outline_panel::ToggleFocus),
        MenuItem::action("Collab Panel", collab_ui::collab_panel::ToggleFocus),
        MenuItem::action("Terminal Panel", terminal_view::panel::Toggle),
        MenuItem::action(
            "Debugger Panel",
            aacode_actions::debug_panel::ToggleFocus,
        ),
    ];

    if !project::DisableAiSettings::get_global(cx).disable_ai {
        view_items.push(MenuItem::action(
            "Agent Panel",
            aacode_actions::assistant::ToggleFocus,
        ));
    }

    view_items.extend([
        MenuItem::action("Git Panel", aacode_actions::git_panel::ToggleFocus),
        MenuItem::separator(),
        MenuItem::action("Diagnostics", diagnostics::Deploy),
        MenuItem::separator(),
    ]);

    if release_channel::ReleaseChannel::try_global(cx)
        == Some(release_channel::ReleaseChannel::Dev)
    {
        view_items.push(MenuItem::action(
            "Toggle GPUI Inspector",
            aacode_actions::dev::ToggleInspector,
        ));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            name: "aacode".into(),
            disabled: false,
            items: vec![
                MenuItem::action("About aacode", aacode_actions::About),
                MenuItem::action("Check for Updates", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu::new("Settings").items([
                    MenuItem::action("Open Settings", aacode_actions::OpenSettings),
                    MenuItem::action("Open Settings File", aacode_actions::OpenSettingsFile),
                    MenuItem::action(
                        "Open Project Settings",
                        aacode_actions::OpenProjectSettings,
                    ),
                    // TODO(aacode): Zed 另有 "Open Project Settings File" 与
                    // "Open Default Settings" 两项；aacode 目前没有对应 action，
                    // 等补上后替换下面的占位。
                    MenuItem::action("Open Project Settings File", NoAction),
                    MenuItem::action("Open Default Settings", NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Open Keymap", aacode_actions::OpenKeymap),
                    MenuItem::action("Open Keymap File", aacode_actions::OpenKeymapFile),
                    MenuItem::action(
                        "Open Default Key Bindings",
                        aacode_actions::OpenDefaultKeymap,
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Select Theme...",
                        aacode_actions::theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        "Select Icon Theme...",
                        aacode_actions::icon_theme_selector::Toggle::default(),
                    ),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu("Services", gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Extensions", aacode_actions::Extensions::default()),
                // TODO(aacode): Zed 用 install_cli::InstallCliBinary；aacode 没有
                // install_cli crate，先用占位，补 crate 后替换。
                #[cfg(not(target_os = "windows"))]
                MenuItem::action("Install CLI", NoAction),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide aacode", Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Others", HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("Show All", ShowAll),
                MenuItem::separator(),
                MenuItem::action("Quit aacode", aacode_actions::Quit),
            ],
        },
        Menu {
            name: "File".into(),
            disabled: false,
            items: vec![
                MenuItem::action("New", workspace::NewFile),
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("Open File...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "Open Folder..."
                    } else {
                        "Open…"
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action("Open Recent…", aacode_actions::OpenRecent::default()),
                MenuItem::action("Open Remote…", aacode_actions::OpenRemote::default()),
                MenuItem::separator(),
                MenuItem::action("Add Folder to Project…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("Save", workspace::Save { save_intent: None }),
                MenuItem::action("Save As…", workspace::SaveAs),
                MenuItem::action("Save All", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "Close Editor",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("Close Project", workspace::CloseProject),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("Undo", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Copy and Trim", editor::actions::CopyAndTrim),
                MenuItem::os_action("Paste", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Find", search::buffer_search::Deploy::find()),
                MenuItem::action("Find in Project", workspace::DeploySearch::default()),
                MenuItem::separator(),
                MenuItem::action(
                    "Toggle Line Comment",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "Selection".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(
                    "Select All",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("Expand Selection", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("Shrink Selection", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action("Select Next Sibling", editor::actions::SelectNextSyntaxNode),
                MenuItem::action(
                    "Select Previous Sibling",
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Add Cursor Above",
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "Add Cursor Below",
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "Select Next Occurrence",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    "Select Previous Occurrence",
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action("Select All Occurrences", editor::actions::SelectAllMatches),
                MenuItem::separator(),
                MenuItem::action("Move Line Up", editor::actions::MoveLineUp),
                MenuItem::action("Move Line Down", editor::actions::MoveLineDown),
                MenuItem::action("Duplicate Selection", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "View".into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: "Go".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Back", workspace::GoBack),
                MenuItem::action("Forward", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(
                    "Command Palette...",
                    aacode_actions::command_palette::Toggle,
                ),
                MenuItem::separator(),
                MenuItem::action("Go to File...", workspace::ToggleFileFinder::default()),
                // TODO(aacode): Zed 有 "Go to Symbol in Project"（project_symbols::Toggle），
                // aacode 没有 project_symbols crate，补 crate 后启用。
                MenuItem::action(
                    "Go to Symbol in Editor...",
                    aacode_actions::outline::ToggleOutline,
                ),
                MenuItem::action("Go to Line/Column...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(
                    "Go to Definition",
                    editor::actions::GoToDefinition::default(),
                ),
                MenuItem::action(
                    "Go to Declaration",
                    editor::actions::GoToDeclaration::default(),
                ),
                MenuItem::action(
                    "Go to Type Definition",
                    editor::actions::GoToTypeDefinition::default(),
                ),
                MenuItem::action(
                    "Find All References",
                    editor::actions::FindAllReferences::default(),
                ),
                // aacode 没有 call_hierarchy crate；同名 action 在 aacode_actions 里。
                MenuItem::action(
                    "Show Incoming Calls",
                    aacode_actions::ShowIncomingCalls,
                ),
                MenuItem::action(
                    "Show Outgoing Calls",
                    aacode_actions::ShowOutgoingCalls,
                ),
                MenuItem::separator(),
                MenuItem::action("Next Problem", editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    "Previous Problem",
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: "Run".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "Spawn Task",
                    aacode_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action("Start Debugger", debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action("Edit tasks.json…", aacode_actions::OpenProjectTasks),
                MenuItem::action("Edit debug.json…", aacode_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action("Continue", debugger_ui::Continue),
                MenuItem::action("Step Over", debugger_ui::StepOver),
                MenuItem::action("Step Into", debugger_ui::StepInto),
                MenuItem::action("Step Out", debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action("Toggle Breakpoint", editor::actions::ToggleBreakpoint),
                MenuItem::action("Edit Breakpoint", editor::actions::EditLogBreakpoint),
                MenuItem::action("Clear All Breakpoints", debugger_ui::ClearAllBreakpoints),
            ],
        },
        Menu {
            name: "Window".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Minimize", Minimize),
                MenuItem::action("Zoom", Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help".into(),
            disabled: false,
            items: vec![
                // TODO(aacode): 品牌项待改。Zed 用 auto_update_ui::ViewReleaseNotesLocally，
                // aacode 没有 auto_update_ui crate，先用占位。
                MenuItem::action("View Release Notes Locally", NoAction),
                MenuItem::action("View Telemetry", aacode_actions::OpenTelemetryLog),
                MenuItem::action("View Dependency Licenses", aacode_actions::OpenLicenses),
                // TODO(aacode): Zed 用 onboarding::ShowWelcome；aacode 的 welcome
                // 在 workspace crate 里，确认导出路径后替换下面的占位。
                MenuItem::action("Show Welcome", NoAction),
                MenuItem::separator(),
                MenuItem::action(
                    "File Bug Report...",
                    aacode_actions::feedback::FileBugReport,
                ),
                MenuItem::action(
                    "Request Feature...",
                    aacode_actions::feedback::RequestFeature,
                ),
                MenuItem::action("Email Us...", aacode_actions::feedback::EmailZed),
                MenuItem::separator(),
                // TODO(aacode): 品牌项待改 —— 以下 URL / 仓库 / 社交账号都是 zed 的。
                MenuItem::action(
                    "Documentation",
                    aacode_actions::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                // TODO(aacode): 品牌项待改 —— Zed 用 feedback::OpenZedRepo，
                // aacode 的 aacode_actions::feedback 只有 EmailZed / FileBugReport /
                // RequestFeature，所以这里先占位，等换成 aacode 仓库地址。
                MenuItem::action("Zed Repository", NoAction),
                MenuItem::action(
                    "Zed Twitter",
                    aacode_actions::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "Join the Team",
                    aacode_actions::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
