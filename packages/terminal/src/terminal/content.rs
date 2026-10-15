use gpui::{AppContext as _, Context, Task, Window};

use crate::alacritty::{
    content_text, last_non_empty_lines, make_content, screen_lines, search_matches, total_lines,
    used_lines,
};
use crate::events::Event;
use crate::types::{
    cell::RenderableCells,
    content::{Content, GridLinesChange},
    point::Range,
    selection::Search,
};

use super::Terminal;

impl Terminal {
    pub fn sync(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let term = self.term.clone();
        let mut terminal = term.lock_unfair();
        //Note that the ordering of events matters for event processing
        while let Some(e) = self.events.pop_front() {
            self.process_terminal_event(&e, &mut terminal, window, cx)
        }

        self.last_content = make_content(&terminal, &self.last_content);
        if self.last_content.grid_lines_change == GridLinesChange::Changed {
            debug_assert!(self.last_content.last_hovered_word.is_none());
            self.refresh_hovered_word(window, cx);

            // Because refresh_hovered_word() may result
            // in new events, but will not trigger a repaint
            if !self.events.is_empty() {
                cx.emit(Event::Wakeup);
            }
        }
    }

    pub fn with_renderable_cells<R>(&self, f: impl for<'a> FnOnce(RenderableCells<'a>) -> R) -> R {
        let term = self.term.lock_unfair();
        let content = term.renderable_content();
        f(RenderableCells::new(content.display_iter))
    }

    pub fn get_content(&self) -> String {
        let term = self.term.lock_unfair();
        content_text(&term)
    }

    pub fn last_n_non_empty_lines(&self, n: usize) -> Vec<String> {
        let terminal = self.term.lock_unfair();
        last_non_empty_lines(&terminal, n)
    }

    pub fn find_matches(&self, searcher: Search, cx: &Context<Self>) -> Task<Vec<Range>> {
        let term = self.term.clone();
        cx.background_spawn(async move {
            let term = term.lock();
            search_matches(&term, searcher)
        })
    }

    pub fn total_lines(&self) -> usize { total_lines(&self.term.lock_unfair()) }

    pub fn viewport_lines(&self) -> usize { screen_lines(&self.term.lock_unfair()) }

    pub fn used_lines(&self) -> usize { used_lines(&self.term.lock_unfair()) }

    pub fn last_content(&self) -> &Content { &self.last_content }
}
