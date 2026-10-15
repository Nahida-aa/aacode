use std::sync::Arc;
use std::time::Duration;

use crate::alacritty::{apply_config, set_default_cursor_style, PtySender};
use crate::cmd::foreground_process_command_from_argv;
use crate::pty_info::PtyProcessInfo;
use crate::terminal_settings::CursorShape as SettingsCursorShape;

use crate::pty_info::ProcessIdGetter;
 // 如果 process_hyperlink 在另一个文件


use super::{ Terminal, }; // task_summary 在 task.rs




/// Separates retained PTY process metadata from resources needed only while
/// the terminal is live.
pub(crate) enum PtyResources {
    Active(PtySender),
    Released,
}

pub(crate) enum TerminalType {
    Pty {
        resources: PtyResources,
        info: Arc<PtyProcessInfo>,
    },
    DisplayOnly,
}


impl Terminal {
    pub fn is_pty(&self) -> bool { matches!(self.terminal_type, TerminalType::Pty { .. }) }

    /// Returns whether this terminal still owns its live PTY sender.
    pub fn has_active_pty_resources(&self) -> bool {
        matches!(
            self.terminal_type,
            TerminalType::Pty {
                resources: PtyResources::Active(_),
                ..
            }
        )
    }

    /// Releases live PTY resources while retaining process metadata and buffered output.
    ///
    /// Calling this method after the resources have already been released is a no-op.
    pub fn release_pty_resources(&mut self) {
        let TerminalType::Pty { resources, info } = &mut self.terminal_type else {
            return;
        };
        let PtyResources::Active(pty_tx) = std::mem::replace(resources, PtyResources::Released)
        else {
            return;
        };
        let info = info.clone();

        pty_tx.shutdown();
        info.terminate_child_process();

        let timer = self.background_executor.timer(Duration::from_millis(100));
        self.background_executor
            .spawn(async move {
                timer.await;
                info.kill_child_process();
            })
            .detach();
    }

    pub fn pid(&self) -> Option<sysinfo::Pid> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => info.pid(),
            TerminalType::DisplayOnly => None,
        }
    }

    pub fn pid_getter(&self) -> Option<&ProcessIdGetter> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => Some(info.pid_getter()),
            TerminalType::DisplayOnly => None,
        }
    }

    /// Normalizes the command name of the foreground process, if one is known.
    pub fn foreground_process_command_name(&self) -> Option<String> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => info
                .current
                .read()
                .as_ref()
                .and_then(|process| foreground_process_command_from_argv(&process.argv)),
            TerminalType::DisplayOnly => None,
        }
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: SettingsCursorShape) {
        set_default_cursor_style(&mut self.term_config, cursor_shape);
        apply_config(&self.term, &self.term_config);
    }
}
