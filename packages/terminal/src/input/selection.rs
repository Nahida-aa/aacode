use crate::alacritty::full_content_range;
use crate::events::InternalEvent;
use crate::grid::geometry::point::{Point, Range};
use crate::pty::Terminal;

#[derive(Clone, Debug)]
pub struct Selection {
    pub(crate) ty: SelectionType,
    pub(crate) start: SelectionAnchor,
    pub(crate) end: SelectionAnchor,
    pub(crate) head: Point,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SelectionAnchor {
    pub(crate) point: Point,
    pub(crate) side: SelectionSide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionSide {
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionType {
    Simple,
    Semantic,
    Lines,
}

impl Selection {
    pub(crate) fn new(selection_type: SelectionType, point: Point, side: SelectionSide) -> Self {
        let anchor = SelectionAnchor { point, side };
        Self {
            ty: selection_type,
            start: anchor,
            end: anchor,
            head: point,
        }
    }

    pub(crate) fn simple_range(range: Range) -> Self {
        let mut selection = Self::new(SelectionType::Simple, range.start(), SelectionSide::Left);
        selection.update(range.end(), SelectionSide::Right);
        selection
    }

    fn update(&mut self, point: Point, side: SelectionSide) {
        self.end = SelectionAnchor { point, side };
        self.head = point;
    }
}

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

#[derive(PartialEq, Eq)]
pub enum SelectionPhase {
    Selecting,
    Ended,
}
