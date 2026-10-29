use std::rc::Rc;

use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;

use super::context::ScrollerContext;
use super::controller::{Effects, Move};
use super::geometry::{Layout, Row, Span};

async fn span(mounted: &Rc<MountedData>) -> Option<Span> {
    let rect = mounted.get_client_rect().await.ok()?;
    Some(Span {
        top: rect.origin.y,
        bottom: rect.origin.y + rect.size.height,
    })
}

impl ScrollerContext {
    pub(super) async fn measure(mut self) -> Option<Layout> {
        let viewport = self.viewport.peek().clone()?;
        let content = self.content.peek().clone()?;
        let handles: Vec<_> = self
            .rows
            .peek()
            .iter()
            .filter_map(|(key, row)| {
                row.mounted
                    .clone()
                    .map(|mounted| (*key, mounted, row.message_id.clone(), row.anchor))
            })
            .collect();
        let viewport_span = span(&viewport).await?;
        let scroll_top = viewport.get_scroll_offset().await.ok()?.y;
        let scroll_height = viewport.get_scroll_size().await.ok()?.height;
        let content_span = span(&content).await?;
        let mut rows = Vec::with_capacity(handles.len());
        for (key, mounted, message_id, anchor) in handles {
            if let Some(span) = span(&mounted).await {
                rows.push(Row {
                    key,
                    message_id,
                    anchor,
                    span,
                });
            }
        }
        rows.sort_by(|a, b| a.span.top.total_cmp(&b.span.top).then(a.key.cmp(&b.key)));
        let spacer_height = self.controller.peek().spacer_height();
        let last_bottom = match self.spacer.peek().clone() {
            Some(spacer) if spacer_height > 0.0 => span(&spacer).await.map(|span| span.bottom),
            _ => rows.last().map(|row| row.span.bottom),
        };
        let stretched = content_span.height() <= viewport_span.height() + 1.0;
        if let (false, Some(bottom)) = (stretched, last_bottom) {
            self.padding_end
                .set((content_span.bottom - bottom).max(0.0));
        }
        Some(Layout {
            viewport: viewport_span,
            scroll_top,
            scroll_height,
            padding_start: rows
                .first()
                .map_or(0.0, |row| (row.span.top - content_span.top).max(0.0)),
            padding_end: *self.padding_end.peek(),
            gap: rows
                .windows(2)
                .map(|pair| pair[1].span.top - pair[0].span.bottom)
                .filter(|gap| *gap >= 0.0)
                .reduce(f64::min)
                .unwrap_or(0.0),
            rows,
        })
    }

    pub(super) fn on_scroll(mut self, layout: &Layout) -> Effects {
        let real_max = layout.scroll_height - layout.client_height();
        self.controller.write().on_scroll(layout, real_max);
        Effects {
            sync_visibility: true,
            ..Effects::default()
        }
    }

    pub(super) fn sync_visibility(mut self, layout: &Layout) {
        if *self.watchers.peek() == 0 {
            return;
        }
        let config = self.controller.peek().config();
        let next = layout.visibility(config.scroll_margin, config.scroll_previous_item_peek);
        if *self.visibility.peek() != next {
            self.visibility.set(next);
        }
    }

    pub(super) async fn perform(self, next: Move) {
        let Some(viewport) = self.viewport.peek().clone() else {
            return;
        };
        let point = PixelsVector2D::new(0.0, next.top);
        let _ = viewport.scroll(point, next.behavior.native()).await;
    }
}
