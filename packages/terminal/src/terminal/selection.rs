use crate::alacritty::full_content_range;
use crate::events::InternalEvent;
use crate::types::point::Range;
use crate::types::selection::Selection;

use super::Terminal;

impl Terminal {
    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some(Selection::simple_range(search_match)));
            if self.vi_mode_enabled {
                self.events
                    .push_back(InternalEvent::MoveViCursorToPoint(search_match.end()));
            } else {
                self.events
                    .push_back(InternalEvent::ScrollToPoint(search_match.start()));
            }
        }
    }

    pub fn select_matches(&mut self, matches: &[Range]) {
        let matches_to_select = self
            .matches
            .iter()
            .filter(|self_match| matches.contains(self_match))
            .cloned()
            .collect::<Vec<_>>();
        for match_to_select in matches_to_select {
            self.set_selection(Some(Selection::simple_range(match_to_select)));
        }
    }

    pub fn select_all(&mut self) {
        let term = self.term.lock();
        let range = full_content_range(&term);
        drop(term);
        self.set_selection(Some(Selection::simple_range(range)));
    }

    fn set_selection(&mut self, selection: Option<Selection>) {
        self.events
            .push_back(InternalEvent::SetSelection(selection));
    }

    pub fn copy(&mut self, keep_selection: Option<bool>) {
        self.events.push_back(InternalEvent::Copy(keep_selection));
    }
}
