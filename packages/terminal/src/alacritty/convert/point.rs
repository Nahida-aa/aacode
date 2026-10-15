use std::ops::RangeInclusive;

use alacritty_terminal::{
    index::{Column, Line, Point as AlacPoint},
    selection::SelectionRange as AlacSelectionRange,
};

use crate::{Point, Range, SelectionRange};

impl Point {
    pub(crate) fn to_alacritty(self) -> AlacPoint {
        AlacPoint::new(Line(self.line), Column(self.column))
    }
}
pub(crate) fn terminal_point_from_alacritty(point: AlacPoint) -> Point {
    Point {
        line: point.line.0,
        column: point.column.0,
    }
}

impl Range {
    pub(crate) fn from_alacritty(range: RangeInclusive<AlacPoint>) -> Self {
        Self {
            start: terminal_point_from_alacritty(*range.start()),
            end: terminal_point_from_alacritty(*range.end()),
        }
    }
}

pub(crate) fn terminal_selection_range_from_alacritty(range: AlacSelectionRange) -> SelectionRange {
    SelectionRange {
        start: terminal_point_from_alacritty(range.start),
        end: terminal_point_from_alacritty(range.end),
        is_block: range.is_block,
    }
}
