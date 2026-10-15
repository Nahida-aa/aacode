use alacritty_terminal::grid::Dimensions as _;
use gpui::{ClipboardItem, Context, Window};
use log::trace;
use settings::Settings as _;
use theme::ActiveTheme as _;

use crate::alacritty::{
    AlacrittyTerm, clear_saved_screen, display_offset, find_from_terminal_point, resize,
    scroll_display, scroll_to_point, selection_text, set_selection as set_term_selection,
    toggle_vi_mode as toggle_term_vi_mode, update_selection as update_term_selection,
    update_selection_to_vi_cursor, update_vi_cursor_for_scroll, vi_goto_point, vi_motion,
};
use crate::events::{Event, InternalEvent, PtyEvent, TerminalBackendEvent};
use crate::mappings::colors::to_vte_rgb;
use crate::mappings::mouse::{grid_point, grid_point_and_side};
use crate::terminal::Terminal;
use crate::terminal::colors::get_color_at_index;
use crate::terminal::pty::{PtyResources, TerminalType};
use crate::terminal_settings::TerminalSettings;
use crate::types::bounds::normalize_terminal_bounds;

impl Terminal {
    pub(crate) fn process_pty_event(&mut self, event: PtyEvent, cx: &mut Context<Self>) {
        match event {
            PtyEvent::Event(event, generation) => {
                if self.output_events_tx.is_none() || generation == self.output_generation {
                    self.process_event(event, cx);
                }
            }
        }
    }

    pub(crate) fn process_event(&mut self, event: TerminalBackendEvent, cx: &mut Context<Self>) {
        match event {
            TerminalBackendEvent::Title(title) => {
                // ignore default shell program title change as windows always sends those events
                // and it would end up showing the shell executable path in breadcrumbs
                #[cfg(windows)]
                if self
                    .shell_program
                    .as_ref()
                    .map(|e| *e == title)
                    .unwrap_or(false)
                {
                    return;
                }

                self.breadcrumb_text = title;
                cx.emit(Event::BreadcrumbsChanged);
            }
            TerminalBackendEvent::ResetTitle => {
                self.breadcrumb_text = String::new();
                cx.emit(Event::BreadcrumbsChanged);
            }
            TerminalBackendEvent::ClipboardStore(data) => {
                cx.write_to_clipboard(ClipboardItem::new_string(data))
            }
            TerminalBackendEvent::ClipboardLoad(format) => {
                self.write_to_pty(
                    match &cx.read_from_clipboard().and_then(|item| item.text()) {
                        // The terminal only supports pasting strings, not images.
                        Some(text) => format(text),
                        _ => format(""),
                    }
                    .into_bytes(),
                )
            }
            TerminalBackendEvent::PtyWrite(out) => self.write_to_pty(out.into_bytes()),
            TerminalBackendEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.last_content.terminal_bounds).into_bytes())
            }
            TerminalBackendEvent::CursorBlinkingChange => {
                let terminal = self.term.lock();
                let blinking = terminal.cursor_style().blinking;
                cx.emit(Event::BlinkChanged(blinking));
            }
            TerminalBackendEvent::Bell => {
                cx.emit(Event::Bell);
            }
            TerminalBackendEvent::Exit => self.register_task_finished(None, cx),
            TerminalBackendEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            TerminalBackendEvent::Wakeup => {
                self.detect_init_command_startup_marker();
                cx.emit(Event::Wakeup);

                if let TerminalType::Pty { info, .. } = &self.terminal_type {
                    info.emit_title_changed_if_changed(cx);
                }
            }
            TerminalBackendEvent::ColorRequest(index, format) => {
                // It's important that the color request is processed here to retain relative order
                // with other PTY writes. Otherwise applications might witness out-of-order
                // responses to requests. For example: An application sending `OSC 11 ; ? ST`
                // (color request) followed by `CSI c` (request device attributes) would receive
                // the response to `CSI c` first.
                // Instead of locking, we could store the colors in `self.last_content`. But then
                // we might respond with out of date value if a "set color" sequence is immediately
                // followed by a color request sequence.

                let color = self.term.lock().colors()[index]
                    .unwrap_or_else(|| to_vte_rgb(get_color_at_index(index, cx.theme().as_ref())));
                self.write_to_pty(format(color).into_bytes());
            }
            TerminalBackendEvent::ChildExit(exit_status) => {
                self.register_task_finished(Some(exit_status), cx);
            }
        }
    }

    pub(crate) fn process_terminal_event(
        &mut self,
        event: &InternalEvent,
        term: &mut AlacrittyTerm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            &InternalEvent::Resize(new_bounds) => {
                let new_bounds = normalize_terminal_bounds(new_bounds);
                trace!("Resizing: new_bounds={new_bounds:?}");

                let columns_changed =
                    self.last_content.terminal_bounds.num_columns() != new_bounds.num_columns();
                self.last_content.terminal_bounds = new_bounds;

                if let TerminalType::Pty {
                    resources: PtyResources::Active(pty_tx),
                    ..
                } = &self.terminal_type
                {
                    pty_tx.resize(new_bounds);
                }

                resize(term, new_bounds);
                if columns_changed {
                    self.reset_cwd_history();
                }
                // If there are matches we need to emit a wake up event to
                // invalidate the matches and recalculate their locations
                // in the new terminal layout
                if !self.matches.is_empty() {
                    cx.emit(Event::Wakeup);
                }
            }
            InternalEvent::Clear => {
                trace!("Clearing");
                clear_saved_screen(term);
                self.reset_cwd_history();
                cx.emit(Event::Wakeup);
            }
            InternalEvent::Scroll(scroll) => {
                trace!("Scrolling: scroll={scroll:?}");
                scroll_display(term, *scroll);
                self.refresh_hovered_word(window, cx);

                if self.vi_mode_enabled {
                    update_vi_cursor_for_scroll(term, *scroll);
                    if let Some(selection_head) = update_selection_to_vi_cursor(term) {
                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                        if let Some(selection_text) = selection_text(term) {
                            cx.write_to_primary(ClipboardItem::new_string(selection_text));
                        }

                        self.selection_head = Some(selection_head);
                        cx.emit(Event::SelectionsChanged)
                    }
                }
            }
            InternalEvent::SetSelection(selection) => {
                trace!("Setting selection: selection={selection:?}");
                set_term_selection(term, selection.as_ref());

                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                if let Some(selection_text) = selection_text(term) {
                    cx.write_to_primary(ClipboardItem::new_string(selection_text));
                }

                if let Some(selection) = selection {
                    self.selection_head = Some(selection.head);
                }
                cx.emit(Event::SelectionsChanged)
            }
            InternalEvent::UpdateSelection(position) => {
                trace!("Updating selection: position={position:?}");
                let (point, side) = grid_point_and_side(
                    *position,
                    self.last_content.terminal_bounds,
                    display_offset(term),
                );

                if update_term_selection(term, point, side) {
                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    if let Some(selection_text) = selection_text(term) {
                        cx.write_to_primary(ClipboardItem::new_string(selection_text));
                    }

                    self.selection_head = Some(point);
                    cx.emit(Event::SelectionsChanged)
                }
            }

            InternalEvent::Copy(keep_selection) => {
                trace!("Copying selection: keep_selection={keep_selection:?}");
                if let Some(txt) = selection_text(term) {
                    cx.write_to_clipboard(ClipboardItem::new_string(txt));
                    if !keep_selection.unwrap_or_else(|| {
                        let settings = TerminalSettings::get_global(cx);
                        settings.keep_selection_on_copy
                    }) {
                        self.events.push_back(InternalEvent::SetSelection(None));
                    }
                }
            }
            InternalEvent::ScrollToPoint(point) => {
                trace!("Scrolling to point: point={point:?}");
                scroll_to_point(term, *point);
                self.refresh_hovered_word(window, cx);
            }
            InternalEvent::MoveViCursorToPoint(point) => {
                trace!("Move vi cursor to point: point={point:?}");
                vi_goto_point(term, *point);
                self.refresh_hovered_word(window, cx);
            }
            InternalEvent::ToggleViMode => {
                trace!("Toggling vi mode");
                self.vi_mode_enabled = !self.vi_mode_enabled;
                toggle_term_vi_mode(term);
            }
            InternalEvent::ViMotion(motion) => {
                trace!("Performing vi motion: motion={motion:?}");
                vi_motion(term, *motion);
            }
            InternalEvent::FindHyperlink(position, open) => {
                trace!("Finding hyperlink at position: position={position:?}, open={open:?}");

                let point = grid_point(
                    *position,
                    self.last_content.terminal_bounds,
                    display_offset(term),
                );

                match find_from_terminal_point(
                    term,
                    point,
                    &mut self.hyperlink_regex_searches,
                    self.path_style,
                ) {
                    Some(hyperlink) => {
                        let history_size = term.history_size();
                        self.process_hyperlink(hyperlink, *open, history_size, cx);
                    }
                    None => {
                        self.clear_hyperlink(cx);
                    }
                }
            }
            InternalEvent::ProcessHyperlink(hyperlink, open) => {
                // history_size must be read here since process_hyperlink cannot lock term
                // (sync() already holds the lock when dispatching events)
                let history_size = term.history_size();
                self.process_hyperlink(hyperlink.clone(), *open, history_size, cx);
            }
        }
    }
}
