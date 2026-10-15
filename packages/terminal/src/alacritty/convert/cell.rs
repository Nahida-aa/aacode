use alacritty_terminal::{
    grid::GridIterator,
    term::cell::{Cell as AlacCell, Flags, Hyperlink as AlacHyperlink},
    vte::ansi::Color,
};

use crate::{Cell, Hyperlink, HyperlinkData, IndexedCell, RenderableCells};

use super::point::terminal_point_from_alacritty;

// 自由函数
pub(crate) fn terminal_hyperlink_from_alacritty(hyperlink: AlacHyperlink) -> Hyperlink {
    Hyperlink::from_alacritty(hyperlink)
}
impl Hyperlink {
    pub fn uri(&self) -> &str {
        match &self.data {
            HyperlinkData::Alacritty(hyperlink) => hyperlink.uri(),
            HyperlinkData::Owned { uri, .. } => uri,
        }
    }

    fn from_alacritty(hyperlink: AlacHyperlink) -> Self {
        Self {
            data: HyperlinkData::Alacritty(hyperlink),
        }
    }
}
impl From<Hyperlink> for AlacHyperlink {
    fn from(hyperlink: Hyperlink) -> Self {
        match hyperlink.data {
            HyperlinkData::Alacritty(hyperlink) => hyperlink,
            HyperlinkData::Owned { id, uri } => Self::new(id.as_deref(), uri.to_string()),
        }
    }
}
pub(crate) fn terminal_cell_from_alacritty(cell: &AlacCell) -> Cell { Cell { cell: cell.clone() } }

// trait impl（必须能看见类型定义所在模块）
impl Cell {
    #[inline]
    pub fn character(&self) -> char { self.cell.c }

    #[inline]
    pub fn foreground(&self) -> Color { self.cell.fg }

    #[inline]
    pub fn background(&self) -> Color { self.cell.bg }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> { self.cell.zerowidth() }

    #[inline]
    pub fn hyperlink(&self) -> Option<Hyperlink> {
        self.cell.hyperlink().map(terminal_hyperlink_from_alacritty)
    }

    #[inline]
    pub fn is_inverse(&self) -> bool { self.cell.flags.contains(Flags::INVERSE) }

    #[inline]
    pub fn is_wide_char_spacer(&self) -> bool { self.cell.flags.contains(Flags::WIDE_CHAR_SPACER) }

    #[inline]
    pub fn is_dim(&self) -> bool { self.cell.flags.intersects(Flags::DIM) }

    #[inline]
    pub fn has_underline(&self) -> bool { self.cell.flags.intersects(Flags::ALL_UNDERLINES) }

    #[inline]
    pub fn has_undercurl(&self) -> bool { self.cell.flags.contains(Flags::UNDERCURL) }

    #[inline]
    pub fn has_strikeout(&self) -> bool { self.cell.flags.intersects(Flags::STRIKEOUT) }

    #[inline]
    pub fn is_bold(&self) -> bool { self.cell.flags.intersects(Flags::BOLD) }

    #[inline]
    pub fn is_italic(&self) -> bool { self.cell.flags.intersects(Flags::ITALIC) }

    #[inline]
    pub fn has_visible_style_modifier(&self) -> bool {
        self.cell
            .flags
            .intersects(Flags::ALL_UNDERLINES | Flags::INVERSE | Flags::STRIKEOUT)
    }
}
impl<'a> RenderableCells<'a> {
    pub(crate) fn new(cells: GridIterator<'a, AlacCell>) -> Self { Self { cells } }
}
/// 上游 zed 的这个 impl 在 `crates/terminal/src/alacritty.rs`（此处对应
/// `alacritty/conversions.rs`），aacode 移植 `terminal.rs` 时漏掉了它。
///
/// 它不是只给 repl 用的：`TerminalElement::layout_grid` 的入参是
/// `impl Iterator<Item = T>`（zed 与 aacode 签名一致），而 `repl` 等调用方
/// 直接把 `Terminal::with_renderable_cells(|cells| ...)` 给出的 `RenderableCells`
/// 传进去。缺这个 impl 就编不过，且报错信息（`RenderableCells is not an iterator`）
/// 指向调用点而不是缺失处，容易误判成调用方写错。
impl Iterator for RenderableCells<'_> {
    type Item = IndexedCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next().map(|cell| IndexedCell {
            point: terminal_point_from_alacritty(cell.point),
            cell: terminal_cell_from_alacritty(cell.cell),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.cells.size_hint() }
}
