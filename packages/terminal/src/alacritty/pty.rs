use super::listener::ZedListener;
use super::geometry;
use super::{AlacrittyPty, AlacrittyTermLock};
use crate::{PtyEvent, TerminalBounds};
use alacritty_terminal::{
    event::Notify,
    event_loop::{EventLoop, Msg, Notifier},
    tty,
};
use anyhow::{Context as _, Result};
use futures::channel::mpsc::UnboundedSender;
#[cfg(target_os = "windows")]
use std::num::NonZeroU32;
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::{borrow::Cow, sync::Arc};
use std::{io, path::PathBuf};
#[cfg(target_os = "windows")]
use windows::Win32::{Foundation::HANDLE, System::Threading::GetProcessId};

use crate::pty::info::ProcessIdGetter;

pub(crate) struct PtySender {
    pub(super) notifier: Notifier,
}

impl PtySender {
    pub(crate) fn notify(&self, input: impl Into<Cow<'static, [u8]>>) {
        self.notifier.notify(input);
    }

    pub(crate) fn resize(&self, bounds: TerminalBounds) {
        if let Err(error) =
            self.notifier
                .0
                .send(Msg::Resize(geometry::window_size_from_terminal_bounds(
                    bounds,
                )))
        {
            log::error!("failed to resize alacritty pty: {error}");
        }
    }

    pub(crate) fn shutdown(&self) {
        if let Err(error) = self.notifier.0.send(Msg::Shutdown) {
            log::debug!("failed to shut down alacritty pty loop: {error}");
        }
    }
}

#[cfg(unix)]
impl From<&AlacrittyPty> for ProcessIdGetter {
    fn from(pty: &AlacrittyPty) -> Self { Self::new(pty.file().as_raw_fd(), pty.child().id()) }
}

#[cfg(windows)]
impl From<&AlacrittyPty> for ProcessIdGetter {
    fn from(pty: &AlacrittyPty) -> Self {
        let child = pty.child_watcher();
        let handle = child.raw_handle();
        let fallback_pid = child.pid().unwrap_or_else(|| unsafe {
            NonZeroU32::new_unchecked(GetProcessId(HANDLE(handle as _)))
        });

        Self::new(handle as i32, u32::from(fallback_pid))
    }
}

#[cfg(not(windows))]
pub(crate) fn current_child_signal_mask() -> io::Result<tty::SignalMask> {
    tty::SignalMask::current()
}
pub(crate) fn pty_options(
    shell: Option<(String, Vec<String>)>,
    working_directory: Option<PathBuf>,
    env: impl IntoIterator<Item = (String, String)>,
    #[cfg(not(windows))] child_signal_mask: Option<tty::SignalMask>,
    #[cfg(windows)] escape_args: bool,
) -> tty::Options {
    tty::Options {
        shell: shell.map(|(program, args)| tty::Shell::new(program, args)),
        working_directory,
        drain_on_exit: true,
        env: env.into_iter().collect(),
        #[cfg(not(windows))]
        child_signal_mask,
        #[cfg(windows)]
        escape_args,
    }
}

pub(crate) fn open_pty(
    options: &tty::Options,
    bounds: TerminalBounds,
    window_id: u64,
) -> io::Result<AlacrittyPty> {
    tty::new(
        options,
        geometry::window_size_from_terminal_bounds(bounds),
        window_id,
    )
}

pub(crate) fn spawn_event_loop(
    term: Arc<AlacrittyTermLock>,
    events_tx: UnboundedSender<PtyEvent>,
    pty: AlacrittyPty,
    drain_on_exit: bool,
) -> Result<PtySender> {
    let event_loop = EventLoop::new(
        term,
        ZedListener {
            sender: events_tx,
            generation: 0,
        },
        pty,
        drain_on_exit,
        false,
    )
    .context("failed to create event loop")?;
    let pty_tx = event_loop.channel();
    let _io_thread = event_loop.spawn();

    Ok(PtySender {
        notifier: Notifier(pty_tx),
    })
}
