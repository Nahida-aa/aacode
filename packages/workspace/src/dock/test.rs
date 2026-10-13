use super::*;

use gpui::{App, Context, Window, actions, div};

pub struct TestPanel {
    pub position: DockPosition,
    pub zoomed: bool,
    pub active: bool,
    pub focus_handle: FocusHandle,
    pub activation_focus_handle: Option<FocusHandle>,
    pub default_size: Pixels,
    pub flexible: bool,
    pub activation_priority: u32,
}
actions!(test_only, [ToggleTestPanel]);

impl EventEmitter<PanelEvent> for TestPanel {}

impl TestPanel {
    pub fn new(position: DockPosition, activation_priority: u32, cx: &mut App) -> Self {
        Self {
            position,
            zoomed: false,
            active: false,
            focus_handle: cx.focus_handle(),
            activation_focus_handle: None,
            default_size: px(300.),
            flexible: false,
            activation_priority,
        }
    }

    pub fn new_flexible(
        position: DockPosition,
        activation_priority: u32,
        cx: &mut App,
    ) -> Self {
        Self {
            flexible: true,
            ..Self::new(position, activation_priority, cx)
        }
    }

    pub fn new_with_activation_child(
        position: DockPosition,
        activation_priority: u32,
        cx: &mut App,
    ) -> Self {
        Self {
            activation_focus_handle: Some(cx.focus_handle()),
            ..Self::new(position, activation_priority, cx)
        }
    }
}

impl Render for TestPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("test")
            .track_focus(&self.focus_handle(cx))
            .children(self.activation_focus_handle.iter().map(|focus_handle| {
                div().id("test-activation-child").track_focus(focus_handle)
            }))
    }
}

impl Panel for TestPanel {
    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.activation_focus_handle
            .clone()
            .unwrap_or_else(|| self.focus_handle(cx))
    }

    fn persistent_name() -> &'static str {
        "TestPanel"
    }

    fn panel_key() -> &'static str {
        "TestPanel"
    }

    fn position(&self, _window: &Window, _: &App) -> super::DockPosition {
        self.position
    }

    fn position_is_valid(&self, _: super::DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        cx.update_global::<SettingsStore, _>(|_, _| {});
    }

    fn default_size(&self, _window: &Window, _: &App) -> Pixels {
        self.default_size
    }

    fn initial_size_state(&self, _window: &Window, _: &App) -> PanelSizeState {
        PanelSizeState {
            size: None,
            flex: None,
        }
    }

    fn supports_flexible_size(&self) -> bool {
        self.flexible
    }

    fn has_flexible_size(&self, _window: &Window, _: &App) -> bool {
        self.flexible
    }

    fn set_flexible_size(
        &mut self,
        flexible: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.flexible = flexible;
    }

    fn icon(&self, _window: &Window, _: &App) -> Option<ui::IconName> {
        None
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        ToggleTestPanel.boxed_clone()
    }

    fn is_zoomed(&self, _window: &Window, _: &App) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        self.zoomed = zoomed;
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        self.active = active;
    }

    fn activation_priority(&self) -> u32 {
        self.activation_priority
    }
}

impl Focusable for TestPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
