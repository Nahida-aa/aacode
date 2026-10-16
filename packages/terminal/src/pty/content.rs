use crate::alacritty::{
    content_text, last_non_empty_lines, screen_lines, total_lines, used_lines,
};
use crate::grid::content::cell::RenderableCells;
use crate::grid::content::content::Content;

use super::Terminal;

impl Terminal {
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

    pub fn total_lines(&self) -> usize { total_lines(&self.term.lock_unfair()) }

    pub fn viewport_lines(&self) -> usize { screen_lines(&self.term.lock_unfair()) }

    pub fn used_lines(&self) -> usize { used_lines(&self.term.lock_unfair()) }

    pub fn last_content(&self) -> &Content { &self.last_content }
}
