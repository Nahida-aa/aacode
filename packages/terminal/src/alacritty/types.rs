use alacritty_terminal::{
    grid::GridIterator,
    sync::FairMutex,
    term::{
        Config, Term,
        cell::{Cell as AlacCell, Hyperlink as AlacHyperlink},
    },
    tty,
};

use super::listener::ZedListener;

pub(crate) type AlacrittyPty = tty::Pty;
pub(crate) type AlacrittyTerm = Term<ZedListener>;
pub(crate) type AlacrittyTermConfig = Config;
pub(crate) type AlacrittyTermLock = FairMutex<AlacrittyTerm>;
pub(crate) type AlacrittyCell = AlacCell;
pub(crate) type AlacrittyGridIterator<'a> = GridIterator<'a, AlacCell>;
pub(crate) type AlacrittyHyperlink = AlacHyperlink;
