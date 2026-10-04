//! About 窗口 —— 对齐 Zed `crates/zed/src/zed.rs::open_about_window`（L1528-L1740）。
//!
//! Zed 把它放在 `zed.rs`（app crate 根）里；aacode 没有 zed.rs 等价文件，
//! 故落在 `core/`（app 级、与 workspace 无关）。

use gpui::{
    App, AppContext, ClipboardItem, Context, FocusHandle, Focusable, Image, ImageFormat, Render,
    SharedString, Size, Window, WindowBounds, WindowKind, WindowOptions, img, point, px,
};
use gpui_util::ResultExt as _;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use std::sync::Arc;
use ui::{
    Button, ButtonStyle, Color, Headline, Label, LabelSize, Navigable, NavigableEntry, TintColor,
    prelude::*,
};

// TODO(aacode): 图标暂用 zed 的 app-icon，需替换为 aacode 自己的。
fn about_window_icon(release_channel: ReleaseChannel) -> Arc<Image> {
    let bytes: &[u8] = match release_channel {
        ReleaseChannel::Dev => include_bytes!("../../resources/app-icon-dev.png"),
        ReleaseChannel::Nightly => include_bytes!("../../resources/app-icon-nightly.png"),
        ReleaseChannel::Preview => include_bytes!("../../resources/app-icon-preview.png"),
        ReleaseChannel::Stable => include_bytes!("../../resources/app-icon.png"),
    };

    Arc::new(Image::from_bytes(ImageFormat::Png, bytes.to_vec()))
}

struct AboutWindow {
    focus_handle: FocusHandle,
    ok_entry: NavigableEntry,
    copy_entry: NavigableEntry,
    app_icon: Arc<Image>,
    message: SharedString,
    commit: Option<SharedString>,
    full_version: SharedString,
}

impl AboutWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let release_channel = ReleaseChannel::global(cx);
        let release_channel_name = release_channel.display_name();
        let full_version: SharedString = AppVersion::global(cx).to_string().into();
        let version = env!("CARGO_PKG_VERSION");

        let debug = if cfg!(debug_assertions) { "(debug)" } else { "" };
        let message: SharedString = format!("{release_channel_name} {version} {debug}").into();
        let commit = AppCommitSha::try_global(cx)
            .map(|sha| sha.full())
            .filter(|commit| !commit.is_empty())
            .map(SharedString::from);

        Self {
            focus_handle: cx.focus_handle(),
            ok_entry: NavigableEntry::focusable(cx),
            copy_entry: NavigableEntry::focusable(cx),
            app_icon: about_window_icon(release_channel),
            message,
            commit,
            full_version,
        }
    }

    fn copy_details(&self, window: &mut Window, cx: &mut Context<Self>) {
        let content = match self.commit.as_ref() {
            Some(commit) => {
                format!("{}\nCommit: {}\nVersion: {}", self.message, commit, self.full_version)
            }
            None => format!("{}\nVersion: {}", self.message, self.full_version),
        };
        cx.write_to_clipboard(ClipboardItem::new_string(content));
        window.remove_window();
    }
}

impl Render for AboutWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ok_is_focused = self.ok_entry.focus_handle.contains_focused(window, cx);
        let copy_is_focused = self.copy_entry.focus_handle.contains_focused(window, cx);

        Navigable::new(
            v_flex()
                .id("about-window")
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(|_, _: &menu::Cancel, window, _cx| {
                    window.remove_window();
                }))
                .min_w_0()
                .size_full()
                .bg(cx.theme().colors().editor_background)
                .text_color(cx.theme().colors().text)
                .p_4()
                .when(cfg!(target_os = "macos"), |this| this.pt_10())
                .gap_4()
                .text_center()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .gap_2()
                        .items_center()
                        .child(img(self.app_icon.clone()).size_16().flex_none())
                        .child(Headline::new(self.message.clone()))
                        .when_some(self.commit.clone(), |this, commit| {
                            this.child(
                                Label::new("Commit").color(Color::Muted).size(LabelSize::XSmall),
                            )
                            .child(Label::new(commit).size(LabelSize::Small))
                        })
                        .child(
                            Label::new("Version").color(Color::Muted).size(LabelSize::XSmall),
                        )
                        .child(Label::new(self.full_version.clone()).size(LabelSize::Small)),
                )
                .child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .child(
                            div()
                                .flex_1()
                                .track_focus(&self.ok_entry.focus_handle)
                                .on_action(cx.listener(|_, _: &menu::Confirm, window, _cx| {
                                    window.remove_window();
                                }))
                                .child(
                                    Button::new("ok", "OK")
                                        .full_width()
                                        .style(ButtonStyle::OutlinedGhost)
                                        .toggle_state(ok_is_focused)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                        .on_click(cx.listener(|_, _, window, _cx| {
                                            window.remove_window();
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .track_focus(&self.copy_entry.focus_handle)
                                .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                                    this.copy_details(window, cx);
                                }))
                                .child(
                                    Button::new("copy", "Copy")
                                        .full_width()
                                        .style(ButtonStyle::Tinted(TintColor::Accent))
                                        .toggle_state(copy_is_focused)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                        .on_click(cx.listener(|this, _event, window, cx| {
                                            this.copy_details(window, cx);
                                        })),
                                ),
                        ),
                )
                .into_any_element(),
        )
        .entry(self.ok_entry.clone())
        .entry(self.copy_entry.clone())
    }
}

impl Focusable for AboutWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.ok_entry.focus_handle.clone()
    }
}

/// 打开（或激活已存在的）About 窗口。
///
/// 绑定在 `aacode_actions::About` 上，见 `initialize::init`。
pub fn open_about_window(cx: &mut App) {
    // 不重复打开
    if let Some(existing) = cx
        .windows()
        .into_iter()
        .find_map(|w| w.downcast::<AboutWindow>())
    {
        existing
            .update(cx, |about_window, window, cx| {
                window.activate_window();
                about_window.ok_entry.focus_handle.focus(window, cx);
            })
            .log_err();
        return;
    }

    let window_size = Size {
        width: px(440.),
        height: px(300.),
    };

    cx.open_window(
        WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("About aacode".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(12.), px(12.))),
            }),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            is_resizable: false,
            is_minimizable: false,
            kind: WindowKind::Floating,
            app_id: Some(ReleaseChannel::global(cx).app_id().to_owned()),
            ..Default::default()
        },
        |window, cx| {
            let about_window = cx.new(AboutWindow::new);
            let focus_handle = about_window.read(cx).ok_entry.focus_handle.clone();
            window.activate_window();
            focus_handle.focus(window, cx);
            about_window
        },
    )
    .log_err();
}
