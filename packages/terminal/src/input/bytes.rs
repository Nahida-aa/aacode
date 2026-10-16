use alacritty_terminal::grid::Dimensions as _;
use anyhow::{Result, bail};

use crate::alacritty::{RegexSearches, new_term};
use crate::events::{Event, InternalEvent};
use crate::grid::content::content::Content;
use crate::input::scroll::Scroll;
use crate::input::selection::SelectionPhase;
use crate::pty::ops::{PtyResources, TerminalType};
use crate::pty::process::subprocess::convert_lf_to_crlf;
use gpui::{Context, px};
use std::borrow::Cow;
use vte::ansi::{Processor, StdSyncHandler};

use crate::pty::Terminal;

impl Terminal {
    pub fn input(&mut self, input: impl Into<Cow<'static, [u8]>>) {
        self.keyboard_input_sent = true;
        self.complete_init_command_startup_handshake();
        self.write_input(input);
    }

    pub(crate) fn write_input(&mut self, input: impl Into<Cow<'static, [u8]>>) {
        let input = input.into();
        if !self.is_remote_terminal && input.contains(&b'\r') {
            let term = self.term.lock_unfair();
            self.pending_cwd_boundary = Some(Self::scrollback_position(
                term.grid().cursor.point.line.0,
                term.history_size(),
            ));
        }

        self.events.push_back(InternalEvent::Scroll(Scroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));
        #[cfg(any(test, feature = "test-support"))]
        self.input_log.push(input.to_vec());

        self.write_to_pty(input);
    }

    /// Write the Input payload to the PTY, if applicable.
    /// (This is a no-op for display-only terminals.)
    pub(crate) fn write_to_pty(&self, input: impl Into<Cow<'static, [u8]>>) {
        let input = input.into();
        #[cfg(any(test, feature = "test-support"))]
        self.pty_write_log.borrow_mut().push(input.to_vec());
        if let TerminalType::Pty {
            resources: PtyResources::Active(pty_tx),
            ..
        } = &self.terminal_type
        {
            if log::log_enabled!(log::Level::Debug) {
                if let Ok(str) = str::from_utf8(&input) {
                    log::debug!("Writing to PTY: {:?}", str);
                } else {
                    log::debug!("Writing to PTY: {:?}", input);
                }
            }
            pty_tx.notify(input);
        }
    }

    /// Normalizes line endings so text captured outside a PTY starts each line at column zero.
    pub fn write_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        let mut previous_byte_was_cr = false;
        let converted = convert_lf_to_crlf(bytes, &mut previous_byte_was_cr);
        self.write_raw_output(&converted, cx);
    }

    /// Terminal byte streams already contain their control sequences and must not be normalized.
    pub fn write_raw_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        let mut term = self.term.lock();
        self.output_processor
            .get_or_insert_with(Processor::<StdSyncHandler>::new)
            .advance(&mut *term, bytes);
        drop(term);
        self.detect_init_command_startup_marker();
        cx.emit(Event::Wakeup);
    }

    /// Replaces all display-only output while retaining the terminal entity and its dimensions.
    pub fn replace_display_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) -> Result<()> {
        if !matches!(self.terminal_type, TerminalType::DisplayOnly) || self.subprocess.is_some() {
            bail!("cannot replace output of a process-backed terminal");
        }
        let Some(events_tx) = &self.output_events_tx else {
            bail!("display-only terminal has no output event channel");
        };
        let Some(generation) = self.output_generation.checked_add(1) else {
            bail!("display output generation exhausted");
        };
        let bounds = self.last_content.terminal_bounds;
        let term = new_term(
            &self.term_config,
            bounds,
            events_tx.clone(),
            self.template.alternate_scroll,
            generation,
        );
        self.output_generation = generation;
        self.term = term;
        self.output_processor = None;
        // The new emulator already has the latest requested size; older resizes can truncate it.
        self.events.clear();
        self.last_content = Content {
            terminal_bounds: bounds,
            ..Default::default()
        };
        self.last_mouse = None;
        self.mouse_down_position = None;
        self.matches.clear();
        self.selection_head = None;
        self.breadcrumb_text.clear();
        self.scroll_px = px(0.);
        self.next_link_id = 0;
        self.selection_phase = SelectionPhase::Ended;
        self.hyperlink_regex_searches = RegexSearches::new(
            &self.template.path_hyperlink_regexes,
            self.template.path_hyperlink_timeout,
        );
        self.vi_mode_enabled = false;
        self.last_hyperlink_search_position = None;
        self.mouse_down_hyperlink = None;
        self.cwd_history.clear();
        self.pending_cwd_boundary = None;
        self.write_raw_output(bytes, cx);
        cx.emit(Event::BlinkChanged(
            self.term.lock().cursor_style().blinking,
        ));
        cx.emit(Event::BreadcrumbsChanged);
        cx.emit(Event::NewNavigationTarget(None));
        cx.emit(Event::OutputReplaced);
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn take_input_log(&mut self) -> Vec<Vec<u8>> { std::mem::take(&mut self.input_log) }

    #[cfg(any(test, feature = "test-support"))]
    pub fn take_pty_write_log(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(self.pty_write_log.get_mut())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn keyboard_input_sent(&self) -> bool { self.keyboard_input_sent }
}
