use std::collections::HashSet;

use super::geometry::{Layout, SCROLL_POSITION_EPSILON};
use super::types::{Behavior, Config, DefaultScrollPosition, ScrollOptions, Scrollable};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    FollowingBottom,
    FreeScrolling,
    AnchoredToMessage,
    SettlingJump,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub top: f64,
    pub behavior: Behavior,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Effects {
    pub scroll: Option<Move>,
    pub commit: bool,
    pub sync_visibility: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PrependAnchor {
    pub key: u64,
    pub viewport_top: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Controller {
    pub(super) config: Config,
    pub(super) mode: Mode,
    pub(super) last_scroll_top: f64,
    pub(super) autoscrolling: Option<f64>,
    pub(super) spacer_height: f64,
    pub(super) streaming_turn: Option<u64>,
    pub(super) handled_anchors: HashSet<u64>,
    pub(super) row_count: usize,
    pub(super) first_row: Option<u64>,
    pub(super) default_applied: bool,
    pub(super) pending_default: bool,
    pub(super) pending_jump: Option<(String, ScrollOptions)>,
    pub(super) prepend_anchor: Option<PrependAnchor>,
    pub(super) published: Scrollable,
}

impl Controller {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            mode: Mode::FreeScrolling,
            last_scroll_top: 0.0,
            autoscrolling: None,
            spacer_height: 0.0,
            streaming_turn: None,
            handled_anchors: HashSet::new(),
            row_count: 0,
            first_row: None,
            default_applied: false,
            pending_default: config.default_scroll_position.holds_viewport(),
            pending_jump: None,
            prepend_anchor: None,
            published: Scrollable::default(),
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn spacer_height(&self) -> f64 {
        self.spacer_height
    }

    pub fn published(&self) -> Scrollable {
        self.published
    }

    pub fn autoscrolling(&self) -> bool {
        self.autoscrolling.is_some()
    }

    pub fn pending_default(&self) -> bool {
        self.pending_default
    }

    pub fn config(&self) -> Config {
        self.config
    }

    pub fn set_config(&mut self, config: Config, layout: &Layout, effects: &mut Effects) {
        let previous = std::mem::replace(&mut self.config, config);
        if previous.default_scroll_position != config.default_scroll_position {
            self.default_applied = false;
            self.apply_default(layout, effects);
        }
        if previous.auto_scroll != config.auto_scroll {
            if config.auto_scroll && self.mode == Mode::FollowingBottom && self.row_count > 0 {
                self.scroll_to_end(layout, ScrollOptions::default(), effects);
            } else {
                effects.commit = true;
            }
        }
    }

    pub(super) fn mark_default_applied(&mut self) {
        self.default_applied = true;
        self.pending_default = false;
    }

    pub fn commit(&mut self, layout: &Layout) {
        let raw = layout.scrollable(self.config.scroll_edge_threshold);
        self.reconcile_follow(layout.scroll_top, raw);
        self.published = if self.mode == Mode::FollowingBottom {
            Scrollable { end: false, ..raw }
        } else {
            raw
        };
    }

    fn reconcile_follow(&mut self, scroll_top: f64, raw: Scrollable) {
        let scrolled_up = scroll_top < self.last_scroll_top - SCROLL_POSITION_EPSILON;
        self.last_scroll_top = scroll_top;
        let holding = matches!(self.mode, Mode::SettlingJump | Mode::AnchoredToMessage);
        if self.config.auto_scroll && !raw.end && !holding {
            self.mode = Mode::FollowingBottom;
        } else if self.mode == Mode::FollowingBottom
            && raw.end
            && scrolled_up
            && self.autoscrolling.is_none()
        {
            self.mode = Mode::FreeScrolling;
        }
    }

    pub fn user_intent(&mut self) {
        if self.mode != Mode::FreeScrolling {
            self.streaming_turn = None;
            self.mode = Mode::FreeScrolling;
        }
    }

    pub fn on_scroll(&mut self, layout: &Layout, real_max_scroll_top: f64) {
        if let Some(target) = self.autoscrolling {
            let arrived = (layout.scroll_top - target).abs() <= SCROLL_POSITION_EPSILON
                || (target > real_max_scroll_top
                    && layout.scroll_top >= real_max_scroll_top - SCROLL_POSITION_EPSILON);
            if arrived {
                self.autoscrolling = None;
            }
        }
        self.commit(layout);
        self.capture_prepend_anchor(layout);
    }

    pub fn apply_default(&mut self, layout: &Layout, effects: &mut Effects) -> bool {
        if self.default_applied || layout.rows.is_empty() {
            return false;
        }
        let instant = ScrollOptions::behavior(Behavior::Auto);
        match self.config.default_scroll_position {
            DefaultScrollPosition::Start => self.scroll_to_start(layout, instant, effects),
            DefaultScrollPosition::End => self.scroll_to_end(layout, instant, effects),
            DefaultScrollPosition::LastAnchor => match layout.last_scroll_anchor().cloned() {
                Some(anchor)
                    if layout.content_bottom() - layout.element_top(&anchor)
                        > layout.client_height() =>
                {
                    self.anchor_turn(layout, &anchor, effects)
                }
                _ => self.scroll_to_end(layout, instant, effects),
            },
        };
        self.mark_default_applied();
        true
    }

    pub fn on_mounted_empty(&mut self) {
        if self.row_count == 0 && !self.default_applied {
            self.pending_default = false;
        }
    }

    pub(super) fn capture_prepend_anchor(&mut self, layout: &Layout) {
        self.prepend_anchor = layout.first_visible_row().map(|row| PrependAnchor {
            key: row.key,
            viewport_top: layout.viewport_top(row),
        });
    }
}
