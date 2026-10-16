use gpui::{AppContext as _, Context, Task};

use crate::alacritty::{AlacrittySearch, search_matches};
use crate::grid::geometry::point::Range;
use crate::pty::Terminal;

#[derive(Clone, Debug)]
pub struct Search {
    pub(crate) search: AlacrittySearch,
}

impl Terminal {
    pub fn find_matches(&self, searcher: Search, cx: &Context<Self>) -> Task<Vec<Range>> {
        let term = self.term.clone();
        cx.background_spawn(async move {
            let term = term.lock();
            search_matches(&term, searcher)
        })
    }
}
