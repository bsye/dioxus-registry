use super::controller::{Controller, Effects, Mode, Move, PrependAnchor};
use super::geometry::{Layout, Row, SCROLL_POSITION_EPSILON};
use super::types::{Align, Behavior, ScrollOptions};

impl Controller {
    pub(super) fn scroll_to_position(
        &mut self,
        layout: &Layout,
        top: f64,
        behavior: Behavior,
        autoscrolling: bool,
        effects: &mut Effects,
    ) {
        let top = top.max(0.0);
        if (layout.scroll_top - top).abs() <= SCROLL_POSITION_EPSILON {
            effects.commit = true;
            return;
        }
        if autoscrolling {
            self.autoscrolling = Some(top);
        }
        effects.scroll = Some(Move { top, behavior });
    }

    pub fn scroll_to_start(
        &mut self,
        layout: &Layout,
        options: ScrollOptions,
        effects: &mut Effects,
    ) -> bool {
        self.spacer_height = 0.0;
        self.streaming_turn = None;
        self.mode = Mode::FreeScrolling;
        self.scroll_to_position(layout, 0.0, options.behavior, false, effects);
        effects.sync_visibility = true;
        true
    }

    pub fn scroll_to_end(
        &mut self,
        layout: &Layout,
        options: ScrollOptions,
        effects: &mut Effects,
    ) -> bool {
        self.spacer_height = 0.0;
        self.streaming_turn = None;
        self.mode = if self.config.auto_scroll {
            Mode::FollowingBottom
        } else {
            Mode::FreeScrolling
        };
        self.scroll_to_position(
            layout,
            layout.max_scroll_top(),
            options.behavior,
            true,
            effects,
        );
        effects.sync_visibility = true;
        true
    }

    pub(super) fn scroll_to_row(
        &mut self,
        layout: &Layout,
        row: &Row,
        options: ScrollOptions,
        keep_previous_peek: bool,
        effects: &mut Effects,
    ) -> bool {
        let mut margin = options.scroll_margin.unwrap_or(self.config.scroll_margin);
        if keep_previous_peek {
            margin += self.config.scroll_previous_item_peek;
        }
        let top = layout.element_scroll_top(row, options.align, margin);
        self.spacer_height = layout.tail_spacer_height(top);
        self.prepend_anchor = Some(PrependAnchor {
            key: row.key,
            viewport_top: layout.viewport_top(row),
        });
        self.mode = if keep_previous_peek {
            Mode::AnchoredToMessage
        } else {
            Mode::SettlingJump
        };
        self.streaming_turn = keep_previous_peek.then_some(row.key);
        self.scroll_to_position(layout, top, options.behavior, false, effects);
        effects.sync_visibility = true;
        true
    }

    pub(super) fn anchor_turn(
        &mut self,
        layout: &Layout,
        row: &Row,
        effects: &mut Effects,
    ) -> bool {
        let options = ScrollOptions {
            align: Align::Start,
            ..ScrollOptions::default()
        };
        self.scroll_to_row(layout, row, options, true, effects)
    }

    pub(super) fn reanchor(&mut self, layout: &Layout, effects: &mut Effects) -> bool {
        if self.mode != Mode::AnchoredToMessage {
            return false;
        }
        let Some(row) = self.streaming_turn.and_then(|key| layout.row(key)) else {
            return false;
        };
        let row = row.clone();
        self.anchor_turn(layout, &row, effects)
    }

    pub fn scroll_to_message(
        &mut self,
        layout: &Layout,
        message_id: &str,
        options: ScrollOptions,
        effects: &mut Effects,
    ) -> bool {
        let Some(row) = layout.row_by_message(message_id).cloned() else {
            if layout.rows.is_empty() {
                self.pending_jump = Some((message_id.to_string(), options));
                self.mark_default_applied();
                return true;
            }
            return false;
        };
        self.mark_default_applied();
        self.pending_jump = None;
        self.scroll_to_row(layout, &row, options, false, effects)
    }

    pub(super) fn flush_pending_jump(&mut self, layout: &Layout, effects: &mut Effects) -> bool {
        let Some((message_id, options)) = self.pending_jump.clone() else {
            return false;
        };
        let Some(row) = layout.row_by_message(&message_id).cloned() else {
            return false;
        };
        self.pending_jump = None;
        self.mark_default_applied();
        self.scroll_to_row(layout, &row, options, false, effects)
    }
}
