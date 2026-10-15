use crate::TerminalBounds;
use alacritty_terminal::{event::WindowSize, grid::Dimensions};

pub(super) fn window_size_from_terminal_bounds(
    bounds: TerminalBounds,
) -> alacritty_terminal::event::WindowSize {
    WindowSize {
        num_lines: bounds.num_lines() as u16,
        num_cols: bounds.num_columns() as u16,
        cell_width: f32::from(bounds.cell_width()) as u16,
        cell_height: f32::from(bounds.line_height()) as u16,
    }
}

impl Dimensions for TerminalBounds {
    /// Note: this is supposed to be for the back buffer's length,
    /// but we exclusively use it to resize the terminal, which does not
    /// use this method. We still have to implement it for the trait though,
    /// hence, this comment.
    fn total_lines(&self) -> usize { self.screen_lines() }
    fn screen_lines(&self) -> usize { self.num_lines() }
    fn columns(&self) -> usize { self.num_columns() }
}
