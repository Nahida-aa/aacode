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

use gpui::{
    App, AppContext, SharedString, WindowDecorations, px, size,
};
use gpui_platform::application;
use std::sync::Arc;
use theme::ActiveTheme;

fn main() {
    tracing_subscriber::fmt::init();

    // `aa-app --printenv` — shell env 捕获子进程（对齐 Zed main.rs L251-L255）。
    // project/src/environment.rs 的 capture_unix 会 shell exec `<exe> --printenv`
    // 来拿到 JSON env vars。没这个分支 shell env 就全是空的。
    if std::env::args().any(|a| a == "--printenv") {
        util::shell_env::print_env();
        return;
    }

    // —— app.run 外层：db / fs / session（不依赖 gpui App）——
    let app = application().with_assets(aa_gpui_kit_assets::Assets);
    let app_db = db::AppDatabase::new();
    let fs = fs::RealFs::new(None, app.background_executor());
    let session_id = uuid::Uuid::new_v4().to_string();
    let session_task = app.background_executor().spawn(session::Session::new(
        session_id,
        db::kvp::KeyValueStore::from_app_db(&app_db),
    ));

    app.run(|cx: &mut App| {
        cx.set_global(app_db);

        aa_gpui_kit_assets::Assets
            .load_fonts(cx)
            .expect("failed to load embedded fonts");

        // —— 全局 init 链（纯 crate 初始化，不涉及 AppState）——
        // 顺序对齐 Zed: settings::init 必须在 theme_settings::init 之前
        // （theme_settings::init 需要 SettingsStore 存在才能读 ThemeSettings）
        gpui_tokio::init(cx);
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

        // —— LanguageRegistry ——
        let languages = Arc::new(language::LanguageRegistry::new(
            cx.background_executor().clone(),
        ));

        // —— NodeRuntime ——
        // Zed 从 SettingsStore 变化建 watch channel 传 node binary options；
        // aacode 先传空 channel（None 作为 shell_env_loaded_rx，watch::channel(None) 作为 options）。
        let (_node_options_tx, node_options_rx) = watch::channel(None);
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

        // —— Release channel（对齐 Zed main.rs L492）——
        release_channel::init(semver::Version::new(0, 1, 0), cx);

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

        // —— 打开第一个窗口 ——
        // 对齐 Zed main.rs L877-L953:
        // 1. initialize_workspace 先注册 observe_new
        // 2. cx.spawn 异步启动窗口创建（不 block_on！gpui foreground executor 不能嵌套阻塞）
        // 3. cx.activate(true) 让 app 出现在前台
        cx.spawn({
            let app_state = app_state.clone();
            async move |cx| {
                let _ = cx
                    .update(|cx| {
                        workspace::open_new(
                            workspace::OpenOptions::default(),
                            app_state,
                            cx,
                            |_workspace, _window, _cx| {
                                // Zed 原版会在这里调 Editor::new_file 或 Launchpad
                            },
                        )
                    })
                    .await;
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
