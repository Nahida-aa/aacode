use alacritty_terminal::{
    grid::Dimensions as _,
    index::{Boundary, Column, Direction as AlacDirection, Point as AlacPoint},
    term::{
        Term,
        search::{Match, RegexIter, RegexSearch},
    },
};
use util::paths::PathStyle;

use crate::{Point, Range, Search};

use super::AlacrittyTerm;
use super::listener::ZedListener;
use super::hyperlinks::{HyperlinkMatch, RegexSearches};

#[derive(Clone, Debug)]
pub(crate) struct AlacrittySearch {
    pub(crate) search: RegexSearch,
}
impl Search {
    pub fn new(search: &str) -> Option<Self> {
        Some(Self {
            search: AlacrittySearch {
                search: RegexSearch::new(search).ok()?,
            },
        })
    }

    pub(crate) fn into_alacritty(self) -> RegexSearch { self.search.search }
}

pub(crate) fn search_matches(term: &Term<ZedListener>, searcher: Search) -> Vec<Range> {
    let mut searcher = searcher.into_alacritty();
    all_search_matches(term, &mut searcher)
        .map(Range::from_alacritty)
        .collect()
}

fn all_search_matches<'a, T>(
    term: &'a Term<T>,
    regex: &'a mut RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let start = AlacPoint::new(term.grid().topmost_line(), Column(0));
    let end = AlacPoint::new(term.grid().bottommost_line(), term.grid().last_column());
    RegexIter::new(start, end, AlacDirection::Right, term, regex)
}

pub(crate) fn find_from_terminal_point(
    term: &AlacrittyTerm,
    point: Point,
    regex_searches: &mut RegexSearches,
    path_style: PathStyle,
) -> Option<HyperlinkMatch> {
    let point = point.to_alacritty().grid_clamp(term, Boundary::Grid);
    super::hyperlinks::find_from_grid_point(term, point, regex_searches, path_style)
}
