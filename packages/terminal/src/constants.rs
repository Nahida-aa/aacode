use gpui::{Pixels, px};
use std::time::Duration;



// https://github.com/alacritty/alacritty/blob/cb3a79dbf6472740daca8440d5166c1d4af5029e/extra/man/alacritty.5.scd?plain=1#L207-L213
pub(crate) const DEFAULT_SCROLL_HISTORY_LINES: usize = 10_000;
pub const MAX_SCROLL_HISTORY_LINES: usize = 100_000;

/// Minimum pointer movement before a left click begins a selection. This keeps
/// a click that jitters by a pixel or two (such as the window-focusing click)
/// from starting a selection and, with `copy_on_select` enabled, clobbering the
/// clipboard. Mirrors the drag threshold used by gpui's `div` element.
pub(crate) const SELECTION_DRAG_THRESHOLD: f64 = 2.0;

pub(crate) const FIND_HYPERLINK_THROTTLE_PX: Pixels = px(5.0);
pub(crate) const FIND_HYPERLINK_THROTTLE: Duration = Duration::from_millis(100);
