use super::controller::{Controller, Effects, Mode, Move};
use super::geometry::{Layout, SCROLL_POSITION_EPSILON};
use super::types::{Behavior, ScrollOptions};

impl Controller {
    pub fn handle_content_change(&mut self, layout: &Layout) -> Effects {
        let mut effects = Effects::default();
        let previous_count = self.row_count;
        let previous_first = self.first_row;
        self.row_count = layout.rows.len();
        self.first_row = layout.rows.first().map(|row| row.key);
        self.reconcile_content(layout, previous_count, previous_first, &mut effects);
        self.handled_anchors.extend(
            layout
                .rows
                .iter()
                .filter(|row| row.anchor)
                .map(|row| row.key),
        );
        if effects.scroll.is_none() {
            self.capture_prepend_anchor(layout);
        }
        effects
    }

    fn reconcile_content(
        &mut self,
        layout: &Layout,
        previous_count: usize,
        previous_first: Option<u64>,
        effects: &mut Effects,
    ) {
        let instant = ScrollOptions::behavior(Behavior::Auto);
        if self.flush_pending_jump(layout, effects) {
            return;
        }
        if previous_count == 0 {
            if layout.rows.is_empty() {
                self.on_mounted_empty();
            }
            if self.apply_default(layout, effects) {
                return;
            }
            if !layout.rows.is_empty() && self.config.auto_scroll {
                self.scroll_to_end(layout, instant, effects);
                return;
            }
            effects.commit = true;
            effects.sync_visibility = true;
            return;
        }
        let first_index = previous_first
            .and_then(|key| layout.rows.iter().position(|row| row.key == key))
            .unwrap_or(0);
        if self.config.preserve_scroll_on_prepend && first_index > 0 {
            self.restore_prepended_anchor(layout, effects);
            return;
        }
        if layout.rows.len() > previous_count
            && let Some(anchor) = layout.new_scroll_anchor(previous_count).cloned()
        {
            if self.config.auto_scroll
                && self.mode == Mode::FollowingBottom
                && layout.has_multiple_new_scroll_anchors(previous_count)
            {
                self.scroll_to_end(layout, instant, effects);
                return;
            }
            self.anchor_turn(layout, &anchor, effects);
            return;
        }
        if layout.rows.len() == previous_count
            && let Some(anchor) = layout
                .unhandled_scroll_anchor(&self.handled_anchors)
                .cloned()
        {
            self.anchor_turn(layout, &anchor, effects);
            return;
        }
        if self.mode == Mode::FollowingBottom && self.config.auto_scroll {
            self.scroll_to_end(layout, instant, effects);
        } else {
            effects.commit = true;
            effects.sync_visibility = true;
        }
    }

    fn restore_prepended_anchor(&mut self, layout: &Layout, effects: &mut Effects) {
        effects.commit = true;
        effects.sync_visibility = true;
        let Some(anchor) = self.prepend_anchor else {
            return;
        };
        let Some(row) = layout.row(anchor.key) else {
            return;
        };
        let delta = layout.viewport_top(row) - anchor.viewport_top;
        if delta.abs() > SCROLL_POSITION_EPSILON {
            effects.scroll = Some(Move {
                top: layout.scroll_top + delta,
                behavior: Behavior::Auto,
            });
        }
    }

    pub fn handle_resize(&mut self, layout: &Layout) -> Effects {
        let mut effects = Effects::default();
        let instant = ScrollOptions::behavior(Behavior::Auto);
        if self.mode == Mode::FollowingBottom && self.config.auto_scroll {
            self.scroll_to_end(layout, instant, &mut effects);
            return effects;
        }
        let previous_spacer = self.spacer_height;
        if self.reanchor(layout, &mut effects) {
            if self.config.auto_scroll && previous_spacer > 0.0 && self.spacer_height == 0.0 {
                effects = Effects::default();
                self.scroll_to_end(layout, instant, &mut effects);
            }
            return effects;
        }
        effects.commit = true;
        effects.sync_visibility = true;
        effects
    }
}
