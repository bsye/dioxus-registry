mod anchoring;
mod controller;
mod geometry;
mod render;

use super::controller::Controller;
use super::geometry::{Layout, Row, Span};
use super::types::Config;

pub const VIEWPORT: f64 = 400.0;
pub const GAP: f64 = 16.0;
pub const PADDING: f64 = 12.0;

pub struct Spec {
    pub id: &'static str,
    pub height: f64,
    pub anchor: bool,
}

pub fn message(id: &'static str, height: f64) -> Spec {
    Spec {
        id,
        height,
        anchor: false,
    }
}

pub fn turn(id: &'static str, height: f64) -> Spec {
    Spec {
        id,
        height,
        anchor: true,
    }
}

pub fn layout(specs: &[Spec], scroll_top: f64) -> Layout {
    let mut top = PADDING - scroll_top;
    let rows = specs
        .iter()
        .map(|spec| {
            let row = Row {
                key: key_of(spec.id),
                message_id: Some(spec.id.to_string()),
                anchor: spec.anchor,
                span: Span {
                    top,
                    bottom: top + spec.height,
                },
            };
            top += spec.height + GAP;
            row
        })
        .collect::<Vec<_>>();
    let content = rows
        .last()
        .map_or(PADDING * 2.0, |row| row.span.bottom + scroll_top + PADDING);
    Layout {
        viewport: Span {
            top: 0.0,
            bottom: VIEWPORT,
        },
        scroll_top,
        scroll_height: content.max(VIEWPORT),
        padding_start: PADDING,
        padding_end: PADDING,
        gap: GAP,
        rows,
    }
}

pub fn key_of(id: &str) -> u64 {
    id.bytes().fold(7, |hash, byte| hash * 31 + u64::from(byte))
}

pub fn following() -> Controller {
    Controller::new(Config {
        auto_scroll: true,
        ..Config::default()
    })
}

pub fn opened(controller: &mut Controller, rows: &[Spec]) -> f64 {
    let first = layout(rows, 0.0);
    let effects = controller.handle_content_change(&first);
    let top = effects.scroll.map_or(0.0, |next| next.top);
    controller.on_scroll(&layout(rows, top), f64::MAX);
    top
}
