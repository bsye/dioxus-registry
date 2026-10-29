use std::collections::HashSet;

use super::types::{Align, Scrollable, Visibility};

pub const SCROLL_POSITION_EPSILON: f64 = 0.5;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Span {
    pub top: f64,
    pub bottom: f64,
}

impl Span {
    pub fn height(self) -> f64 {
        self.bottom - self.top
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub key: u64,
    pub message_id: Option<String>,
    pub anchor: bool,
    pub span: Span,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    pub viewport: Span,
    pub scroll_top: f64,
    pub scroll_height: f64,
    pub padding_start: f64,
    pub padding_end: f64,
    pub gap: f64,
    pub rows: Vec<Row>,
}

impl Layout {
    pub fn client_height(&self) -> f64 {
        self.viewport.height()
    }

    pub fn row(&self, key: u64) -> Option<&Row> {
        self.rows.iter().find(|row| row.key == key)
    }

    pub fn row_by_message(&self, message_id: &str) -> Option<&Row> {
        self.rows
            .iter()
            .find(|row| row.message_id.as_deref() == Some(message_id))
    }

    pub fn content_bottom(&self) -> f64 {
        self.rows
            .iter()
            .fold(self.padding_start + self.padding_end, |bottom, row| {
                bottom.max(self.element_top(row) + row.span.height() + self.padding_end)
            })
    }

    pub fn element_top(&self, row: &Row) -> f64 {
        row.span.top - self.viewport.top + self.scroll_top
    }

    pub fn viewport_top(&self, row: &Row) -> f64 {
        row.span.top - self.viewport.top
    }

    pub fn max_scroll_top(&self) -> f64 {
        (self.content_bottom() - self.client_height()).max(0.0)
    }

    pub fn scrollable(&self, threshold: f64) -> Scrollable {
        Scrollable {
            start: self.scroll_top > threshold,
            end: self.content_bottom() - self.scroll_top - self.client_height() > threshold,
        }
    }

    pub fn element_scroll_top(&self, row: &Row, align: Align, margin: f64) -> f64 {
        let top = self.element_top(row);
        let height = row.span.height();
        let client = self.client_height();
        match align {
            Align::Start => top - self.padding_start - margin,
            Align::Center => {
                let inset = (client - self.padding_start - self.padding_end).max(0.0);
                top - self.padding_start - (inset - height) / 2.0 - margin
            }
            Align::End => top - client + height + self.padding_end + margin,
            Align::Nearest => self.nearest_scroll_top(top, height, margin),
        }
    }

    fn nearest_scroll_top(&self, top: f64, height: f64, margin: f64) -> f64 {
        let bottom = top + height;
        let visible_top = self.scroll_top + self.padding_start;
        let visible_bottom = self.scroll_top + self.client_height() - self.padding_end;
        if top >= visible_top && bottom <= visible_bottom {
            self.scroll_top
        } else if top < visible_top {
            top - self.padding_start - margin
        } else {
            bottom - self.client_height() + self.padding_end + margin
        }
    }

    pub fn tail_spacer_height(&self, scroll_top: f64) -> f64 {
        (scroll_top + self.client_height() - self.content_bottom() - self.gap)
            .max(0.0)
            .ceil()
    }

    pub fn first_visible_row(&self) -> Option<&Row> {
        self.rows.iter().find(|row| {
            row.message_id.is_some()
                && row.span.bottom > self.viewport.top
                && row.span.top < self.viewport.bottom
        })
    }

    pub fn new_scroll_anchor(&self, previous_count: usize) -> Option<&Row> {
        self.rows.iter().skip(previous_count).find(|row| row.anchor)
    }

    pub fn has_multiple_new_scroll_anchors(&self, previous_count: usize) -> bool {
        self.rows
            .iter()
            .skip(previous_count)
            .filter(|row| row.anchor)
            .count()
            > 1
    }

    pub fn unhandled_scroll_anchor(&self, handled: &HashSet<u64>) -> Option<&Row> {
        self.rows
            .iter()
            .find(|row| row.anchor && !handled.contains(&row.key))
    }

    pub fn last_scroll_anchor(&self) -> Option<&Row> {
        self.rows.iter().rev().find(|row| row.anchor)
    }

    pub fn visibility(&self, margin: f64, peek: f64) -> Visibility {
        let line = self.viewport.top + margin + peek;
        let mut visibility = Visibility::default();
        for row in &self.rows {
            let Some(message_id) = &row.message_id else {
                continue;
            };
            if row.span.bottom > line && row.span.top < self.viewport.bottom {
                visibility.visible_message_ids.push(message_id.clone());
            }
            if row.anchor && row.span.top <= line + SCROLL_POSITION_EPSILON {
                visibility.current_anchor_id = Some(message_id.clone());
            }
        }
        visibility
    }
}
