//! aacode desktop — main entry.
//!
//! 对齐 Zed `crates/zed/src/main.rs` 完整结构：
//! - **app.run 闭包外层**：构建 `AppDatabase`、`RealFs`、启动 `Session::new` async task。
//! - **app.run 闭包内层**：gpui_tokio → theme_settings → settings → editor → terminal → title_bar
//!   → `<dyn Fs>::set_global` → `Client::production` → `LanguageRegistry::new`
//!   → `NodeRuntime::new` → block_on session → `AppSession::new`
//!   → `UserStore::new` / `WorkspaceStore::new`
//!   → `AppState::set_global` → `workspace::init` → `initialize_workspace` → `open_window`。
//! - Sidebar / TitleBar / Panels 全由 `observe_new` 自动注入。

use clap::Parser;
use std::io;
use futures::{FutureExt as _, StreamExt as _, channel::oneshot};
use gpui::{
    App, AppContext, SharedString, Task, WindowDecorations, px, size,
};
use gpui_platform::application;
use git::GitHostingProviderRegistry;
use settings::Settings as _;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Instant;
use theme::ActiveTheme;
use util::ResultExt as _;

/// 启动参数。对齐 Zed `crates/zed/src/main.rs` 的 `Args`（L190-291）。
///
/// Zed 的原始结构有 18 个字段，其中 etw_*（Windows ETW 追踪）、crash_handler、
/// askpass 这几项与 aacode 无关，未搬。`foreground` 与 `dock_action` 是 Windows-only
/// 但仍保留 —— windows_only_instance.rs 的 handle_single_instance 要用它们转发请求。
/// `printenv` 用 clap 字段替代了原先手写的 `args().any(...)` 扫描。
#[derive(Parser, Debug)]
#[command(name = "aacode", disable_version_flag = true, max_term_width = 100)]
struct Args {
    /// A sequence of space-separated paths or urls that you want to open.
    ///
    /// Use `path:line:row` syntax to open a file at a specific location.
    /// Non-existing paths and directories will ignore `:line:row` suffix.
    ///
    /// URLs can either be `file://` or `aacode://` scheme, or relative to the app.
    paths_or_urls: Vec<String>,

    /// Pairs of file paths to diff. Can be specified multiple times.
    /// When directories are provided, recurses into them and shows all changed files in a single multi-diff view.
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["OLD_PATH", "NEW_PATH"])]
    diff: Vec<String>,

    /// Sets a custom directory for all user data (e.g., database, extensions, logs).
    ///
    /// This overrides the default platform-specific data directory location.
    /// On Linux/FreeBSD, the default is `$XDG_DATA_HOME/zed`.
    #[arg(long, value_name = "DIR", verbatim_doc_comment)]
    user_data_dir: Option<String>,

    /// The username and WSL distribution to use when opening paths. If not specified,
    /// aacode will attempt to open the paths directly.
    #[cfg(target_os = "windows")]
    #[arg(long, value_name = "USER@DISTRO")]
    wsl: Option<String>,

    /// Open the project in a dev container.
    #[arg(long)]
    dev_container: bool,

    /// Dump all registered gpui actions as JSON（调试用）。
    #[arg(long)]
    dump_all_actions: bool,

    /// Windows 专用：把控制台附着到父进程（GUI 子系统下让 stdout 可见）。
    #[arg(long)]
    #[cfg(target_os = "windows")]
    #[arg(hide = true)]
    foreground: bool,

    /// Windows 专用：由 explorer 转发来的 dock 右键菜单动作序号。
    #[arg(long)]
    #[cfg(target_os = "windows")]
    #[arg(hide = true)]
    dock_action: Option<usize>,

    /// Outputs environment variables as JSON to stdout。
    ///
    /// project/src/environment.rs 的 capture_unix 会 shell exec `<exe> --printenv`
    /// 来拿子进程的 env vars（Zed main.rs L251-255）。
    #[arg(long)]
    printenv: bool,
}

/// 进程启动时刻，供 miniprofiler_ui 计算「启动到首次交互」的耗时。
/// 对齐 Zed main.rs L199 / L202（在 main() 首行 get_or_init）。
static STARTUP_TIME: OnceLock<Instant> = OnceLock::new();

fn main() {
    STARTUP_TIME.get_or_init(Instant::now);
    tracing_subscriber::fmt::init();

    // 启动参数。对齐 Zed main.rs L213 `let args = Args::parse();`。
    let args = Args::parse();

    // `aacode --printenv` — shell env 捕获子进程（对齐 Zed main.rs L251-L255）。
    // project/src/environment.rs 的 capture_unix 会 shell exec `<exe> --printenv`
    // 来拿到 JSON env vars。没这个分支 shell env 就全是空的。
    if args.printenv {
        util::shell_env::print_env();
        return;
    }

    if args.dump_all_actions {
        dump_all_gpui_actions();
        return;
    }

    // 自定义数据目录要在建 db / session 之前设好。对齐 Zed main.rs L262-270。
    let restart_arguments = if let Some(directory) = args.user_data_dir.as_deref() {
        let directory = paths::set_custom_data_dir(directory);
        vec![
            std::ffi::OsString::from("--user-data-dir"),
            directory.as_os_str().to_owned(),
        ]
    } else {
        Vec::new()
    };

    // git_hosting_providers::init 会读 GlobalGitHostingProviderRegistry 全局，
    // 必须在 app.run 之前把 registry 造好。对齐 Zed main.rs L423。
    let git_hosting_provider_registry =
        Arc::new(GitHostingProviderRegistry::new());

    // —— app.run 外层：db / fs / session（不依赖 gpui App）——
    let app = application()
        .with_assets(aa_gpui_kit_assets::Assets)
        .with_restart_arguments(restart_arguments);
    let app_db = db::AppDatabase::new();
    let fs = fs::RealFs::new(None, app.background_executor());
    let session_id = uuid::Uuid::new_v4().to_string();
    let session_task = app.background_executor().spawn(session::Session::new(
        session_id,
        db::kvp::KeyValueStore::from_app_db(&app_db),
    ));

    // —— CLI IPC 监听 ——
    // 对齐 Zed main.rs L358-368。OpenListener 是「打开请求」的总入口：
    // 既接启动时 argv 里的路径，也接后续 cli 二进制经 unix socket 转发来的请求。
    // 无它则 `--new` / desktop 的 [Desktop Action NewWorkspace] 全部无效
    // （open_listener 里 listen_for_cli_connections / handle_cli_connection 此前
    // 都只有定义、没有调用点）。
    let (open_listener, mut open_rx) =
        aa_app_lib::core::open_listener::OpenListener::new();

    // single-instance 检查：listen 失败说明已有实例占着 socket。对齐 Zed L360-368。
    // windows/macos 的 handle_single_instance 依赖各自平台模块，aacode 未移植，
    // 那两个平台下本变量恒为 false（即允许多实例）。
    let failed_single_instance_check = if *aa_env_vars::ZED_STATELESS
        || *release_channel::RELEASE_CHANNEL == release_channel::ReleaseChannel::Dev
    {
        false
    } else {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            aa_app_lib::core::open_listener::listen_for_cli_connections(
                open_listener.clone(),
            )
            .is_err()
        }
        #[cfg(target_os = "windows")]
        {
            !aa_app_lib::core::windows_only_instance::handle_single_instance(
                open_listener.clone(),
                &args,
            )
        }

        #[cfg(target_os = "macos")]
        {
            use aa_app_lib::core::mac_only_instance::*;
            ensure_only_instance() != IsOnlyInstance::Yes
        }
    };

    // 已有实例在跑 → 提示后退出，把打开请求让给首个实例处理。对齐 Zed L382-385。
    // 少了这段，第二个实例仍会继续启动并开自己的窗口（socket bind 失败只是被记下），
    // 结果是同一项目开两个互不相干的窗口。
    if failed_single_instance_check {
        println!("aacode is already running");
        return;
    }

    app.run(move |cx: &mut App| {
        cx.set_global(app_db);

        aa_gpui_kit_assets::Assets
            .load_fonts(cx)
            .expect("failed to load embedded fonts");

        // —— 全局 init 链（纯 crate 初始化，不涉及 AppState）——
        // 顺序对齐 Zed: settings::init 必须在 theme_settings::init 之前
        // （theme_settings::init 需要 SettingsStore 存在才能读 ThemeSettings）
        gpui_tokio::init(cx);

        // Release channel 必须早于任何读 GlobalReleaseChannel 的 init —— 例如
        // ad_credentials_provider::global 会按 Dev/Release 决定用系统 keychain 还是
        // development 文件。原先放在 app_state 之后（第 339 行），导致
        // git_hosting_providers::init 读不到该全局而 panic。对齐 Zed main.rs L492。
        release_channel::init(semver::Version::new(0, 1, 0), cx);
        settings::init(cx);
        // 绑定内置默认快捷键（default-<os>.json + base_keymap + vim）。
        // 必须在 settings::init 之后——它要读 BaseKeymap 全局。
        // 缺这步的话应用里一个默认快捷键都没有：backspace / ctrl-a / ctrl-s
        // 之类全部静默失效（编辑器自身 on_key_down 的少数键仍可用）。
        aa_app_lib::initialize::load_default_keymap(cx);
        // 传资产源：theme_settings 据此装载内嵌的 `themes/**/*.json`
        // （内置 Catppuccin 主题 —— 语法高亮的 102 个 capture 都来自那份 JSON）。
        theme_settings::init(
            theme::LoadThemes::All(Box::new(aa_gpui_kit_assets::Assets)),
            cx,
        );
        editor::init(cx);
        terminal_view::init(cx);
        title_bar::init(cx); // 内部 observe_new(|ws| ws.set_titlebar_item)
        theme_selector::init(cx); // 注册 Toggle action handler
        // 对齐 Zed main.rs：注册 zed_prompt_renderer，覆盖 gpui 的 fallback
        // 弹窗（否则关闭未保存文件等 prompt 会是朴素白框样式）。
        ui_prompt::init(cx);

        // —— 扩展子系统（对齐 Zed main.rs L524-L525 / L561 / L566）——
        // extension::init 建 ExtensionHost 的全局 Entity；ExtensionHostProxy::global
        // 取出它的 Arc 供后续各 extension crate 注册自己的 proxy。
        // 整块此前完全没接线：debug_adapter_extension / language_extension /
        // extension_host / theme_extension 都拿不到 proxy → 扩展机制静默失效。
        extension::init(cx);
        let extension_host_proxy = extension::ExtensionHostProxy::global(cx);

        // —— 其余「只需 &mut App」的 crate init ——
        // 这一批 crate 已全部 fork 进仓且有 pub fn init，但此前没有任何调用点：
        // 它们不进 inventory → 内置 keymap 里的绑定被静默跳过 → action 触发无反应，
        // 且全程无编译错误无 panic（见 .agents/memory/zed-init-chain-gaps.md）。
        // 顺序对齐 Zed main.rs（括号内为 zed 的行号）。
        // git_hosting_providers::init 读 GlobalGitHostingProviderRegistry 全局，
        // 必须先 set_global —— 对齐 Zed main.rs L519。
        GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);
        git_hosting_providers::init(cx); // L520
        debugger_tools::init(cx); // L592
        command_palette::init(cx); // L677
        acp_tools::init(cx); // L701
        edit_prediction_ui::init(cx); // L704
        web_search::init(cx); // L705
        // 需 client + user_store，故排在 app_state 之前的 client 创建之后插；
        // 但 zed 用的是 app_state，故这里延后到 app_state 构造后（见下）。
        snippet_provider::init(cx); // L707
        recent_projects::init(cx); // L726
        dev_container::init(cx); // L727
        image_viewer::init(cx); // L734
        diagnostics::init(cx); // L736
        audio::init(cx); // L738
        go_to_line::init(cx); // L742
        file_finder::init(cx); // L743
        outline::init(cx); // L745
        call_hierarchy::init(cx); // L746
        tasks_ui::init(cx); // L750
        search::init(cx); // L753
        lsp_locations::init(cx); // L754
        vim::init(cx); // L764
        encoding_selector::init(cx); // L767
        language_selector::init(cx); // L768
        line_ending_selector::init(cx); // L769
        toolchain_selector::init(cx); // L771
        language_tools::init(cx); // L774
        feedback::init(cx); // L779
        markdown_preview::init(cx); // L780
        svg_preview::init(cx); // L782
        edit_prediction::init(cx); // L787
        json_schema_store::init(cx); // L789
        miniprofiler_ui::init(*STARTUP_TIME.get().unwrap(), cx); // L790
        which_key::init(cx); // L791

        // —— Panel init（对齐 Zed zed.rs L6174-6176）——
        git_ui::init(cx);
        project_panel::init(cx);
        outline_panel::init(cx);
        // 注册 aacode_actions::OpenKeymap 的 handler：标题栏「用户」菜单里的
        // Keymap 项、welcome 里的 Keymap 卡片都 dispatch 这个 action，没有它
        // 点击就毫无反应（面板类型已实现，缺的是这里的注册，见 zed main.rs L785）。
        keymap_editor::init(cx);
        // tab_switcher / lsp_command_selector 的 crate 已 fork 进仓，但原先没有任何
        // crate 依赖它们 → 未被链接进本二进制 → inventory 里没有它们的 action →
        // 内置 keymap 中对应的绑定（ctrl-tab / tab / ...）解析时被跳过。
        // 对齐 Zed main.rs L744 / L770，由顶层 app 依赖以避免与 workspace 循环。
        tab_switcher::init(cx);
        lsp_command_selector::init(cx);
        // repl / tabular_data_preview 同理：action 要进 inventory 才能被内置 keymap
        // 解析（repl::Run、tabular_data::OpenPreview*）。对齐 Zed main.rs L725 / L781。
        tabular_data_preview::init(cx);

        // —— Fs 全局 ——
        <dyn fs::Fs>::set_global(fs.clone(), cx);

        // repl::init 需要 Arc<dyn Fs>，故排在 fs 全局之后。对齐 Zed main.rs L725。
        repl::init(fs.clone(), cx);

        // —— HTTP client（对齐 Zed main.rs L508-L528）——
        // 先给 gpui 一个 ReqwestClient，Client::production 内部要用；
        // 再用 client.http_client()（带 server URL 前缀的 HttpClientWithUrl）覆盖。
        let user_agent = format!("aacode/{}", env!("CARGO_PKG_VERSION"));
        let http = {
            let _guard = gpui_tokio::Tokio::handle(cx).enter();
            reqwest_client::ReqwestClient::proxy_and_user_agent(None, &user_agent)
                .expect("could not start HTTP client")
        };
        cx.set_http_client(Arc::new(http));

        // —— Client ——
        let client = client::Client::production(cx);
        cx.set_http_client(client.http_client());
        // 注册 client 自身的 action（SignIn / SignOut 等）。对齐 Zed main.rs L593。
        client::init(&client, cx);
        // dap_adapters 注册 DAP adapter 相关 action。对齐 Zed main.rs L659。
        dap_adapters::init(cx);
        // auto_update 注册检查更新 / 安装更新的 handler。对齐 Zed main.rs L658。
        auto_update::init(client.clone(), cx);

        // —— LanguageRegistry ——
        let mut languages = language::LanguageRegistry::new(cx.background_executor().clone());
        // LSP server 的下载目录。扩展提供的 LSP（yaml-language-server、gopls 等）
        // 要下载二进制到这里，没设就报
        // `Failed to start language server "xxx": no language server download dir defined`。
        // 对齐 Zed main.rs L530。
        languages.set_language_server_download_dir(paths::languages_dir().clone());
        let languages = Arc::new(languages);

        // —— NodeRuntime ——
        // 对齐 Zed main.rs L533-L559。
        //
        // 关键：options 的 watch::Sender 必须被 observe_global 闭包**捕获**以保持存活。
        // 之前这里写的是 `let (_node_options_tx, rx) = watch::channel(None);` —— 下划线
        // 前缀让它立即 drop，且值恒为 None 永不填充。于是 NodeRuntime::instance()
        // 第一次 borrow() 拿不到值 → options.changed().await → sender 已死 →
        // Err(NoSenderError)("sender was dropped") → 返回 UnavailableNodeRuntime。
        // 后果：prettier 安装必失败、所有依赖 node 的 LSP 起不来，且只有 ERROR 日志、
        // 不崩溃。详见 .agents/zed-modifications.md。
        let (mut node_options_tx, node_options_rx) = watch::channel(None);
        cx.observe_global::<settings::SettingsStore>(move |cx| {
            let settings = &project::ProjectSettings::get_global(cx).node;
            let options = node_runtime::NodeBinaryOptions {
                allow_path_lookup: !settings.ignore_system_version,
                // TODO: Expose this setting（与 Zed 同）
                allow_binary_download: true,
                use_paths: settings.path.as_ref().map(|node_path| {
                    let node_path = PathBuf::from(shellexpand::tilde(node_path).as_ref());
                    let npm_path = settings
                        .npm_path
                        .as_ref()
                        .map(|path| PathBuf::from(shellexpand::tilde(&path).as_ref()));
                    (
                        node_path.clone(),
                        npm_path.unwrap_or_else(|| {
                            let base_path = PathBuf::new();
                            node_path.parent().unwrap_or(&base_path).join("npm")
                        }),
                    )
                }),
            };
            node_options_tx.send(Some(options)).log_err();
        })
        .detach();
        // 让每个新窗口的 scrollbar 监听 SettingsStore 变化。对齐 Zed main.rs L557。
        ui::on_new_scrollbars::<settings::SettingsStore>(cx);
        let node_runtime =
            node_runtime::NodeRuntime::new(client.http_client(), None, node_options_rx);

        // —— Language init（对齐 Zed main.rs L562）——
        // 注册 native grammars（markdown, rust, python, ...）+ LSP adapters
        languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);

        // —— Session ——
        let session = cx.foreground_executor().block_on(session_task);
        let session = cx.new(|cx| session::AppSession::new(session, cx));

        // —— UserStore / WorkspaceStore ——
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));

        // —— Client 全局 ——
        client::Client::set_global(client.clone(), cx);

        // —— 扩展子系统续（对齐 Zed main.rs L561 / L566 / L662）——
        // 三个 extension crate 各注册一种 proxy 到 extension_host_proxy；
        // extension_host::init 建 ExtensionStore（装扩展、管理扩展进程）。
        // 都必须排在 fs / client / node_runtime 就绪之后。
        debug_adapter_extension::init(extension_host_proxy.clone(), cx);
        language_extension::init(
            language_extension::LspAccess::ViaWorkspaces({
                let workspace_store = workspace_store.clone();
                Arc::new(move |cx: &mut gpui::App| {
                    workspace_store.update(cx, |workspace_store, cx| {
                        Ok(workspace_store
                            .workspaces()
                            .filter_map(|weak| weak.upgrade())
                            .map(|workspace: gpui::Entity<workspace::Workspace>| {
                                workspace.read(cx).project().read(cx).lsp_store()
                            })
                            .collect())
                    })
                })
            }),
            extension_host_proxy.clone(),
            languages.clone(),
        );
        extension_host::init(
            extension_host_proxy.clone(),
            fs.clone(),
            client.clone(),
            node_runtime.clone(),
            cx,
        );

        // —— Collab 初始化链（对齐 Zed zed.rs L6166-6168）——
        channel::init(&client, user_store.clone(), cx);
        call::init(client.clone(), user_store.clone(), cx);
        notifications::init(client.clone(), user_store.clone(), cx);

        // —— AppState 构造 ——
        let app_state = Arc::new(workspace::AppState {
            languages,
            client,
            user_store,
            workspace_store,
            fs,
            build_window_options: |_, cx| {
                gpui::WindowOptions {
                    window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                        None,
                        size(1100.0.into(), px(720.0)),
                        cx,
                    ))),
                    window_decorations: Some(WindowDecorations::Client),
                    window_background: cx.theme().window_background_appearance(),
                    titlebar: Some(gpui::TitlebarOptions {
                        appears_transparent: true,
                        title: Some(SharedString::from("aacode")),
                        ..Default::default()
                    }),
                    // Linux/FreeBSD 下设置 X11 窗口图标（_NET_WM_ICON）。由 build.rs 的
                    // prepare_app_icon_x11 把 resources/app-icon.png 缩到 256x256 写进
                    // OUT_DIR/app_icon.png，这里读回来。对齐 Zed
                    // crates/zed/src/zed.rs L378-388（APP_ICON 静态）+ L414（icon 字段）。
                    // 没有它 Linux 窗口没有应用图标，任务栏/Alt-Tab 走 gpui 的兜底图标。
                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    icon: app_icon(),
                    ..Default::default()
                }
            },
            node_runtime,
            session,
        });
        workspace::AppState::set_global(app_state.clone(), cx);

        // 注册 dev::ToggleInspector 的 handler（app menu「View → Toggle GPUI
        // Inspector」与 ctrl-alt-i 走这个 action）。debug 构建下真的打开
        // inspector；release 下只提示「仅 debug 构建可用」并从命令面板隐藏。
        // 需要 AppState，故排在 set_global 之后。对齐 Zed main.rs L788。
        inspector_ui::init(app_state.clone(), cx);

        // —— Agent init 链（对齐 Zed main.rs L694-L722，AppState 之后）——
        language_model::init(cx);
        // 对齐 Zed main.rs L695：必须在 language_models::init 之前注册 LLM token
        // 全局监听器。language_models/src/provider/cloud.rs:120 会调
        // RefreshLlmTokenListener::global(cx) 读这个全局，少这步启动即 panic
        // （gpui app.rs: "no state of type client::llm_token::GlobalRefreshLlmTokenListener exists"）。
        client::RefreshLlmTokenListener::register(
            app_state.client.clone(),
            app_state.user_store.clone(),
            cx,
        );
        // 补齐 Zed main.rs L700 的 provider 注册：language_model::init 只建
        // LanguageModelRegistry 这个 trait/类型容器，不注册任何 provider。缺了
        // 这行，registry.visible_providers() 恒为空 → Settings → LLM Providers
        // 整页空白（settings_ui/src/pages/llm_providers_page.rs:31），且不报错。
        // 依赖 app_state 已构造完成：user_store / client 都取自 app_state，与 Zed
        // 的 `app_state.user_store.clone()` / `app_state.client.clone()` 一致。
        language_models::init(
            app_state.user_store.clone(),
            app_state.client.clone(),
            cx,
        );
        // copilot_ui 注册 Copilot 面板与相关 action。对齐 Zed main.rs L693。
        // copilot_chat 必须排在 copilot_ui 之前：后者要读 GlobalCopilotChat 全局。
        // 对齐 Zed main.rs L678-L691。credentials_provider 在 aacode 对应
        // ad_credentials_provider（Zed 侧叫 zed_credentials_provider）。
        let copilot_chat_configuration = copilot_chat::CopilotChatConfiguration {
            enterprise_uri: language::language_settings::all_language_settings(None, cx)
                .edit_predictions
                .copilot
                .enterprise_uri
                .clone(),
        };
        let credentials_provider = ad_credentials_provider::global(cx);
        copilot_chat::init(
            app_state.client.http_client(),
            credentials_provider,
            copilot_chat_configuration,
            cx,
        );
        copilot_ui::init(&app_state, cx);
        // zed L706 web_search_providers / L766 journal / L786 extensions_ui
        // 三者都需要 app_state（或其中的 client / user_store），故集中在此。
        web_search_providers::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        journal::init(app_state.clone(), cx);
        extensions_ui::init(cx);
        let prompt_builder = prompt_store::PromptBuilder::load(app_state.fs.clone(), false, cx);
        project::AgentRegistryStore::init_global(
            cx,
            app_state.fs.clone(),
            app_state.client.http_client(),
        );
        agent_ui::init(
            app_state.fs.clone(),
            prompt_builder,
            app_state.languages.clone(),
            false, // is_new_install
            false, // is_eval
            cx,
        );

        // —— Workspace 全局 action ——
        workspace::init(app_state.clone(), cx);

        // —— Settings UI（对齐 Zed main.rs L784）——
        settings_ui::init(cx);

        // —— Collab UI（需要 &Arc<AppState>，对齐 Zed zed.rs L6173）——
        collab_ui::init(&app_state, cx);

        // —— Debugger ——
        debugger_ui::init(cx);

        // —— observe_new 注册 ——
        aa_app_lib::initialize::initialize_workspace(app_state.clone(), cx);

        // —— 设置应用菜单（application_menu 靠 cx.get_menus() 读取数据）——
        // 对齐 Zed：菜单体系由 app_menus.rs 构建；App 级 action 的 handler 在这里注册。
        // 放在 open_window 之前因为 title_bar::init 已在上面 observe 了 Workspace。
        // 对齐 Zed main.rs L587 `zed::init(cx)`：注册 App 级 action handler。
        aa_app_lib::initialize::init(cx);
        // 对齐 Zed main.rs：`auto_update_ui::init` 注册 Release Notes 本地查看 /
        // 更新通知的 handler；`onboarding::init` 注册 ShowWelcome / OpenOnboarding 等。
        auto_update_ui::init(cx);
        onboarding::init(cx);
        // 对齐 Zed main.rs L856-L857：先取菜单再 set，避免 `&mut App` 借用冲突。
        let menus = aa_app_lib::app_menus::app_menus(cx);
        cx.set_menus(menus);

        // —— 语法高亮：把当前主题灌进语言注册表（对齐 Zed main.rs L830-L839）——
        // LanguageRegistryState.theme 为 None 时，每个加载的语言都会跳过
        // `language.set_theme`，highlight map 全空 → 编辑器没有语法高亮。
        app_state.languages.set_theme(cx.theme().clone());
        cx.observe_global::<theme::GlobalTheme>({
            let languages = app_state.languages.clone();
            move |cx| {
                languages.set_theme(cx.theme().clone());
            }
        })
        .detach();

        // —— 主题变化时更新所有窗口的 background_appearance（对齐 Zed main.rs L795-L829）——
        // 必须在 open_window 之前注册，这样第一个窗口创建后主题变化也能生效。
        cx.observe_global::<theme::GlobalTheme>(|cx| {
            let background_appearance = cx.theme().window_background_appearance();
            for &mut window in cx.windows().iter_mut() {
                window
                    .update(cx, |_, window, _| {
                        window.set_background_appearance(background_appearance)
                    })
                    .ok();
            }
        })
        .detach();

        // —— 启动参数 → RawOpenRequest（对齐 Zed main.rs L884-914）——
        let urls: Vec<_> = args
            .paths_or_urls
            .iter()
            .map(|arg| aa_app_lib::core::parse_url_arg(arg, cx))
            .collect();
        let diff_all_mode = args
            .diff
            .chunks(2)
            .any(|pair| PathBuf::from(&pair[0]).is_dir() || PathBuf::from(&pair[1]).is_dir());
        let diff_paths: Vec<[String; 2]> = args
            .diff
            .chunks(2)
            .map(|chunk| [chunk[0].clone(), chunk[1].clone()])
            .collect();
        #[cfg(target_os = "windows")]
        let wsl = args.wsl;
        #[cfg(not(target_os = "windows"))]
        let wsl = None;

        if !urls.is_empty() || !diff_paths.is_empty() {
            open_listener.open(aa_app_lib::core::open_listener::RawOpenRequest {
                urls,
                diff_paths,
                wsl,
                diff_all: diff_all_mode,
                dev_container: args.dev_container,
                ..Default::default()
            })
        }

        // GC 需要知道「本次 session」与「上次 session」的 id，才能判断哪些 workspace
        // 记录还该保留（对齐 Zed main.rs L919-924）。
        let (current_session_id, last_session_id) = {
            let session = app_state.session.read(cx);
            (
                session.id().to_owned(),
                session.last_session_id().map(|id| id.to_owned()),
            )
        };

        // —— 打开第一个窗口：恢复上次会话（对齐 Zed main.rs L929-953）——
        //
        // 原先这里是无条件 workspace::open_new(Default::default())，闭包还是空的
        // （注释写着「Zed 原版会在这里调 Editor::new_file 或 Launchpad」），
        // 结果每次启动都是全新空窗口、上次的项目与窗口布局全丢。
        // Zed 是三分支：启动参数里有 focus-only 请求 → 恢复；带其他请求 → 交
        // handle_open_request；没有请求 → 恢复。
        let restore_task = match open_rx
            .try_recv()
            .ok()
            .and_then(|request| {
                aa_app_lib::core::open_listener::OpenRequest::parse(request, cx).log_err()
            }) {
            Some(request) if request.is_focus_app_only() => cx.spawn({
                let app_state = app_state.clone();
                async move |cx| {
                    if let Err(e) = aa_app_lib::core::restore_or_create_workspace(app_state, cx)
                        .await
                    {
                        aa_app_lib::core::fail_to_open_window_async(e, cx);
                    }
                }
            }),
            Some(request) => {
                aa_app_lib::core::handle_open_request(request, app_state.clone(), cx);
                Task::ready(())
            }
            None => cx.spawn({
                let app_state = app_state.clone();
                async move |cx| {
                    if let Err(e) = aa_app_lib::core::restore_or_create_workspace(app_state, cx)
                        .await
                    {
                        aa_app_lib::core::fail_to_open_window_async(e, cx);
                    }
                }
            }),
        };

        // —— 后续 CLI 请求循环（对齐 Zed main.rs L987-1002）——
        // 首个窗口就位前先等 restore_finished / first_window_placed，否则
        // macOS 冷启动的 `aacode <path>` 会因看不到已恢复的窗口而多开一个（zed#61346）。
        let (first_window_tx, first_window_rx) = oneshot::channel::<()>();
        let first_window_tx = std::rc::Rc::new(std::cell::RefCell::new(Some(first_window_tx)));
        let _first_window_subscription = cx.observe_new::<workspace::MultiWorkspace>(move |_, _, _| {
            if let Some(tx) = first_window_tx.borrow_mut().take() {
                tx.send(()).ok();
            }
        });
        let restore_finished = cx.background_spawn(restore_task).shared();
        cx.spawn({
            let restore_finished = restore_finished.clone();
            let app_state = app_state.clone();
            async move |cx| {
                let _first_window_subscription = _first_window_subscription;
                let first_window_placed = first_window_rx.shared();
                while let Some(request) = open_rx.next().await {
                    futures::select_biased! {
                        _ = restore_finished.clone() => {}
                        _ = first_window_placed.clone() => {}
                    }
                    cx.update(|cx| {
                        if let Some(request) =
                            aa_app_lib::core::open_listener::OpenRequest::parse(request, cx).log_err()
                        {
                            aa_app_lib::core::handle_open_request(request, app_state.clone(), cx);
                        }
                    });
                }
            }
        })
        .detach();

        // 恢复完成后清理未被任何 session 引用的 workspace 记录（对齐 Zed L968+）。
        cx.spawn({
            let db = workspace::WorkspaceDb::global(cx);
            let fs = app_state.fs.clone();
            let restore_finished = restore_finished.clone();
            async move |_cx| {
                restore_finished.await;
                db.garbage_collect_workspaces(
                    fs.as_ref(),
                    &current_session_id,
                    last_session_id.as_deref(),
                )
                .await
                .log_err();
            }
        })
        .detach();

        cx.activate(true);

        // 启动后异步检查 MultiWorkspace 窗口是否可被 read
        cx.spawn(async move |cx| {
            cx.update(|cx| {
                tracing::info!("=== startup post-check ===");
                tracing::info!("total windows: {}", cx.windows().len());
                tracing::info!("active_window: {}", cx.active_window().is_some());
                for (i, window) in cx.windows().iter().enumerate() {
                    let downcast = window.downcast::<workspace::MultiWorkspace>();
                    tracing::info!("  window[{}] downcast:<MultiWorkspace>={}", i, downcast.is_some());
                    if let Some(handle) = downcast {
                        let read_result = handle.read(cx);
                        tracing::info!("  window[{}] .read() is_ok={}", i, read_result.is_ok());
                    }
                }
            });
            cx.background_executor()
                .timer(std::time::Duration::from_millis(500))
                .await;
            cx.update(|cx| {
                tracing::info!("=== startup post-check (+500ms) ===");
                tracing::info!("active_window: {}", cx.active_window().is_some());
                for (i, window) in cx.windows().iter().enumerate() {
                    let downcast = window.downcast::<workspace::MultiWorkspace>();
                    tracing::info!("  window[{}] downcast:<MultiWorkspace>={}", i, downcast.is_some());
                    if let Some(handle) = downcast {
                        let read_result = handle.read(cx);
                        tracing::info!("  window[{}] .read() is_ok={}", i, read_result.is_ok());
                    }
                }
            });
        })
        .detach();
    });
}

/// 把所有已注册的 gpui action 导出为 JSON（调试用）。
/// 对齐 Zed `crates/zed/src/main.rs:2008`。
fn dump_all_gpui_actions() {
    #[derive(Debug, serde::Serialize)]
    struct ActionDef {
        name: &'static str,
        human_name: String,
        schema: Option<serde_json::Value>,
        deprecated_aliases: &'static [&'static str],
        deprecation_message: Option<&'static str>,
        documentation: Option<&'static str>,
    }
    let mut generator = settings::KeymapFile::action_schema_generator();
    let mut actions = gpui::generate_list_of_all_registered_actions()
        .map(|action| {
            let schema = (action.json_schema)(&mut generator)
                .map(|s| serde_json::to_value(s).expect("Failed to serialize action schema"));
            ActionDef {
                name: action.name,
                human_name: command_palette::humanize_action_name(action.name),
                schema,
                deprecated_aliases: action.deprecated_aliases,
                deprecation_message: action.deprecation_message,
                documentation: action.documentation,
            }
        })
        .collect::<Vec<ActionDef>>();

    actions.sort_by_key(|a| a.name);

    let schema_definitions = serde_json::to_value(generator.definitions())
        .expect("Failed to serialize schema definitions");

    let output = serde_json::json!({
        "actions": actions,
        "schema_definitions": schema_definitions,
    });

    io::Write::write(
        &mut std::io::stdout(),
        serde_json::to_string_pretty(&output).unwrap().as_bytes(),
    )
    .unwrap();
}

/// Linux/FreeBSD 的窗口图标（X11 `_NET_WM_ICON`）。
///
/// 对齐 Zed `crates/zed/src/zed.rs:378-388`。图片由 `build.rs` 的
/// `prepare_app_icon_x11` 生成到 `OUT_DIR/app_icon.png`（256x256），
/// 这里 `include_bytes!` 读回并解码；解码失败只记日志返回 None —— 图标不是
/// 关键路径，不该因此让窗口开不出来。
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn app_icon() -> Option<Arc<image::RgbaImage>> {
    static APP_ICON: std::sync::LazyLock<Option<Arc<image::RgbaImage>>> =
        std::sync::LazyLock::new(|| {
            // build.rs 已确保这个文件存在并解码过。
            const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/app_icon.png"));
            util::maybe!({
                let image = image::ImageReader::new(std::io::Cursor::new(BYTES))
                    .with_guessed_format()?
                    .decode()?
                    .into();
                anyhow::Ok(Arc::new(image))
            })
            .log_err()
        });
    APP_ICON.as_ref().cloned()
}
