use alacritty_terminal::term::TermMode;

use crate::Modes;

impl Modes {
    #[cfg(test)]
    pub(super) fn to_alacritty(self) -> TermMode {
        let mut mode = TermMode::empty();
        add_alacritty_mode(&mut mode, self, Self::APP_CURSOR, TermMode::APP_CURSOR);
        add_alacritty_mode(&mut mode, self, Self::APP_KEYPAD, TermMode::APP_KEYPAD);
        add_alacritty_mode(&mut mode, self, Self::SHOW_CURSOR, TermMode::SHOW_CURSOR);
        add_alacritty_mode(&mut mode, self, Self::LINE_WRAP, TermMode::LINE_WRAP);
        add_alacritty_mode(&mut mode, self, Self::ORIGIN, TermMode::ORIGIN);
        add_alacritty_mode(&mut mode, self, Self::INSERT, TermMode::INSERT);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::LINE_FEED_NEW_LINE,
            TermMode::LINE_FEED_NEW_LINE,
        );
        add_alacritty_mode(&mut mode, self, Self::FOCUS_IN_OUT, TermMode::FOCUS_IN_OUT);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::ALTERNATE_SCROLL,
            TermMode::ALTERNATE_SCROLL,
        );
        add_alacritty_mode(
            &mut mode,
            self,
            Self::BRACKETED_PASTE,
            TermMode::BRACKETED_PASTE,
        );
        add_alacritty_mode(&mut mode, self, Self::SGR_MOUSE, TermMode::SGR_MOUSE);
        add_alacritty_mode(&mut mode, self, Self::UTF8_MOUSE, TermMode::UTF8_MOUSE);
        add_alacritty_mode(&mut mode, self, Self::ALT_SCREEN, TermMode::ALT_SCREEN);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::MOUSE_REPORT_CLICK,
            TermMode::MOUSE_REPORT_CLICK,
        );
        add_alacritty_mode(&mut mode, self, Self::MOUSE_DRAG, TermMode::MOUSE_DRAG);
        add_alacritty_mode(&mut mode, self, Self::MOUSE_MOTION, TermMode::MOUSE_MOTION);
        add_alacritty_mode(&mut mode, self, Self::VI, TermMode::VI);
        mode
    }
}

pub(crate) fn terminal_modes_from_alacritty(mode: TermMode) -> Modes {
    let mut terminal_modes = Modes::empty();
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::APP_CURSOR,
        Modes::APP_CURSOR,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::APP_KEYPAD,
        Modes::APP_KEYPAD,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::SHOW_CURSOR,
        Modes::SHOW_CURSOR,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::LINE_WRAP,
        Modes::LINE_WRAP,
    );
    add_terminal_mode(&mut terminal_modes, mode, TermMode::ORIGIN, Modes::ORIGIN);
    add_terminal_mode(&mut terminal_modes, mode, TermMode::INSERT, Modes::INSERT);
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::LINE_FEED_NEW_LINE,
        Modes::LINE_FEED_NEW_LINE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::FOCUS_IN_OUT,
        Modes::FOCUS_IN_OUT,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::ALTERNATE_SCROLL,
        Modes::ALTERNATE_SCROLL,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::BRACKETED_PASTE,
        Modes::BRACKETED_PASTE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::SGR_MOUSE,
        Modes::SGR_MOUSE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::UTF8_MOUSE,
        Modes::UTF8_MOUSE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::ALT_SCREEN,
        Modes::ALT_SCREEN,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_REPORT_CLICK,
        Modes::MOUSE_REPORT_CLICK,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_DRAG,
        Modes::MOUSE_DRAG,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_MOTION,
        Modes::MOUSE_MOTION,
    );
    add_terminal_mode(&mut terminal_modes, mode, TermMode::VI, Modes::VI);
    terminal_modes
}

fn add_terminal_mode(
    terminal_modes: &mut Modes,
    alacritty_modes: TermMode,
    alacritty_mode: TermMode,
    terminal_mode: Modes,
) {
    if alacritty_modes.contains(alacritty_mode) {
        terminal_modes.insert(terminal_mode);
    }
}
#[cfg(test)]
fn add_alacritty_mode(
    alacritty_modes: &mut TermMode,
    terminal_modes: Modes,
    terminal_mode: Modes,
    alacritty_mode: TermMode,
) {
    if terminal_modes.contains(terminal_mode) {
        alacritty_modes.insert(alacritty_mode);
    }
}
