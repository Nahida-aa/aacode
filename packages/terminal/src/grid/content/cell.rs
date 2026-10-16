use std::ops::Deref;

use crate::alacritty::{AlacrittyCell, AlacrittyGridIterator};

use crate::grid::geometry::point::Point;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    pub(crate) cell: AlacrittyCell,
}
pub struct RenderableCells<'a> {
    pub(crate) cells: AlacrittyGridIterator<'a>,
}
#[derive(Debug, Clone)]
pub struct IndexedCell {
    pub point: Point,
    pub cell: Cell,
}

impl Deref for IndexedCell {
    type Target = Cell;

    #[inline]
    fn deref(&self) -> &Cell { &self.cell }
}
