use alacritty_terminal::{
    grid::Dimensions as _,
    index::{Column, Point as AlacPoint},
    term::Term,
};

use crate::grid::geometry::point::Range;

use super::listener::ZedListener;

pub(crate) fn total_lines(term: &Term<ZedListener>) -> usize { term.total_lines() }

pub(crate) fn screen_lines(term: &Term<ZedListener>) -> usize { term.screen_lines() }
pub(crate) fn full_content_range(term: &Term<ZedListener>) -> Range {
    let start = AlacPoint::new(term.topmost_line(), Column(0));
    let end = AlacPoint::new(term.bottommost_line(), term.last_column());
    Range::from_alacritty(start..=end)
}
