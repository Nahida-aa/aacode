use std::sync::Arc;

use futures::channel::mpsc::UnboundedSender;

use alacritty_terminal::event::{Event as AlacTermEvent, EventListener};

use crate::{PtyEvent, events::TerminalBackendEvent};

use super::geometry::window_size_from_terminal_bounds;

#[derive(Clone)]
pub(crate) struct ZedListener {
    pub(super) sender: UnboundedSender<PtyEvent>,
    pub(super) generation: u64,
}

// Alacritty's `Term` holds this listener (its `event_proxy` field) and calls
// `send_event` on it whenever something happens in the emulator — the method
// name is from the sender's (Term's) perspective; the listener itself is a
// passive sink, mirroring `vte::ansi::Handler::input`.
impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.sender
            .unbounded_send(PtyEvent::Event(event.into(), self.generation))
            .ok();
    }
}

impl From<AlacTermEvent> for TerminalBackendEvent {
    fn from(event: AlacTermEvent) -> Self {
        match event {
            AlacTermEvent::MouseCursorDirty => Self::MouseCursorDirty,
            AlacTermEvent::Title(title) => Self::Title(title),
            AlacTermEvent::ResetTitle => Self::ResetTitle,
            AlacTermEvent::ClipboardStore(_, data) => Self::ClipboardStore(data),
            AlacTermEvent::ClipboardLoad(_, format) => Self::ClipboardLoad(format),
            AlacTermEvent::ColorRequest(index, format) => Self::ColorRequest(index, format),
            AlacTermEvent::PtyWrite(output) => Self::PtyWrite(output),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                Self::TextAreaSizeRequest(Arc::new(move |bounds| {
                    format(window_size_from_terminal_bounds(bounds))
                }))
            }
            AlacTermEvent::CursorBlinkingChange => Self::CursorBlinkingChange,
            AlacTermEvent::Wakeup => Self::Wakeup,
            AlacTermEvent::Bell => Self::Bell,
            AlacTermEvent::Exit => Self::Exit,
            AlacTermEvent::ChildExit(status) => Self::ChildExit(status),
        }
    }
}
