use crate::alacritty::AlacrittySearch;

use super::point::{Point, Range};

#[derive(Clone, Debug)]
pub struct Search {
    pub(crate) search: AlacrittySearch,
}

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

#[derive(PartialEq, Eq)]
pub enum SelectionPhase {
    Selecting,
    Ended,
}
