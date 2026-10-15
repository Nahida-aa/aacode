use std::ops::Deref;
use std::sync::Arc;

use crate::alacritty::{AlacrittyCell, AlacrittyGridIterator, AlacrittyHyperlink};

use super::point::Point;

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hyperlink {
    pub data: HyperlinkData,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HyperlinkData {
    Alacritty(AlacrittyHyperlink),
    Owned { id: Option<Arc<str>>, uri: Arc<str> },
}
