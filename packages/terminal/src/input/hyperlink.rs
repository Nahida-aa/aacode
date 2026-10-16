use std::time::Instant;

use crate::alacritty::HyperlinkMatch;
use crate::constants::{FIND_HYPERLINK_THROTTLE, FIND_HYPERLINK_THROTTLE_PX};
use crate::{
    events::{Event, InternalEvent, MaybeNavigationTarget, PathLikeTarget},
    grid::content::HoveredWord,
    grid::geometry::point::{Point, Range},
    input::selection::SelectionPhase,
};
use gpui::{Context, Modifiers, Pixels, Point as GpuiPoint, Window}; // 如果 process_hyperlink 在另一个文件

use crate::pty::Terminal;

use crate::alacritty::find_from_terminal_point;

impl Terminal {
    pub(crate) fn process_hyperlink(
        &mut self,
        hyperlink: HyperlinkMatch,
        open: bool,
        history_size: usize,
        cx: &mut Context<Self>,
    ) {
        let HyperlinkMatch {
            text: maybe_url_or_path,
            is_url,
            range,
        } = hyperlink;
        let prev_hovered_word = self.last_content.last_hovered_word.take();
        let match_line = range.start().line;
        let working_directory = self.cwd_at_line(match_line, history_size);

        let target = if is_url {
            if let Some(path) = maybe_url_or_path.strip_prefix("file://") {
                let decoded_path = urlencoding::decode(path)
                    .map(|decoded| decoded.into_owned())
                    .unwrap_or(path.to_owned());

                MaybeNavigationTarget::PathLike(PathLikeTarget {
                    maybe_path: decoded_path,
                    working_directory,
                })
            } else {
                MaybeNavigationTarget::Url(maybe_url_or_path.clone())
            }
        } else {
            MaybeNavigationTarget::PathLike(PathLikeTarget {
                maybe_path: maybe_url_or_path.clone(),
                working_directory,
            })
        };

        if open {
            cx.emit(Event::Open(target));
        } else {
            self.update_selected_word(prev_hovered_word, range, maybe_url_or_path, target, cx);
        }
    }

    pub(crate) fn clear_hyperlink(&mut self, cx: &mut Context<Self>) {
        if self.last_content.last_hovered_word.take().is_some() {
            cx.emit(Event::NewNavigationTarget(None));
        }
    }

    pub(crate) fn find_hyperlink_at_point(&mut self, point: Point) -> Option<HyperlinkMatch> {
        let term_lock = self.term.lock();
        find_from_terminal_point(
            &term_lock,
            point,
            &mut self.hyperlink_regex_searches,
            self.path_style,
        )
    }

    fn update_selected_word(
        &mut self,
        prev_word: Option<HoveredWord>,
        word_match: Range,
        word: String,
        navigation_target: MaybeNavigationTarget,
        cx: &mut Context<Self>,
    ) {
        if let Some(prev_word) = prev_word
            && prev_word.word == word
            && prev_word.word_match == word_match
        {
            self.last_content.last_hovered_word = Some(prev_word);
            return;
        }

        self.last_content.last_hovered_word = Some(HoveredWord {
            word,
            word_match,
            id: self.next_link_id(),
        });
        cx.emit(Event::NewNavigationTarget(Some(navigation_target)));
        cx.notify()
    }

    fn next_link_id(&mut self) -> usize {
        let res = self.next_link_id;
        self.next_link_id = self.next_link_id.wrapping_add(1);
        res
    }

    pub(crate) fn schedule_find_hyperlink(
        &mut self,
        modifiers: Modifiers,
        position: GpuiPoint<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if self.selection_phase == SelectionPhase::Selecting
            || !modifiers.secondary()
            || !self.last_content.terminal_bounds.bounds.contains(&position)
        {
            self.clear_hyperlink(cx);
            return;
        }

        // Throttle hyperlink searches to avoid excessive processing
        let now = Instant::now();
        let throttle = !self
            .last_hyperlink_search_position
            .map_or(true, |last_pos| {
                // Only search if mouse moved significantly or enough time passed
                let distance_moved = ((position.x - last_pos.x).abs()
                    + (position.y - last_pos.y).abs())
                    > FIND_HYPERLINK_THROTTLE_PX;
                let time_elapsed =
                    now.duration_since(self.last_mouse_move_time) > FIND_HYPERLINK_THROTTLE;
                distance_moved || time_elapsed
            });

        #[cfg(test)]
        let throttle = if self.suppress_hyperlink_throttle_once {
            self.suppress_hyperlink_throttle_once = false;
            false
        } else {
            throttle
        };

        if throttle {
            return;
        }

        self.last_mouse_move_time = now;
        self.last_hyperlink_search_position = Some(position);
        self.events.push_back(InternalEvent::FindHyperlink(
            position - self.last_content.terminal_bounds.bounds.origin,
            false,
        ));
        cx.notify();
    }

    pub(crate) fn refresh_hovered_word(&mut self, window: &Window, cx: &mut Context<Self>) {
        self.schedule_find_hyperlink(window.modifiers(), window.mouse_position(), cx);
    }
}
