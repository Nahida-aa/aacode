use alacritty_terminal::{term::RenderableCursor, vte::ansi::CursorShape as AlacCursorShape};

use crate::{Cursor, CursorShape};

use super::point::terminal_point_from_alacritty;

impl Cursor {
    pub(crate) fn from_alacritty(cursor: RenderableCursor) -> Self {
        Self {
            shape: terminal_cursor_shape_from_alacritty(cursor.shape),
            point: terminal_point_from_alacritty(cursor.point),
        }
    }
}

pub(crate) fn terminal_cursor_shape_from_alacritty(shape: AlacCursorShape) -> CursorShape {
    match shape {
        AlacCursorShape::Block => CursorShape::Block,
        AlacCursorShape::Underline => CursorShape::Underline,
        AlacCursorShape::Beam => CursorShape::Bar,
        AlacCursorShape::HollowBlock => CursorShape::HollowBlock,
        AlacCursorShape::Hidden => CursorShape::Hidden,
    }
}
