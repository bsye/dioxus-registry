use std::collections::BTreeMap;
use std::future::Future;
use std::rc::Rc;

use dioxus::core::{Runtime, current_scope_id};
use dioxus::prelude::*;

use super::controller::{Controller, Effects, Move};
use super::types::{Config, Scrollable, Visibility};

#[derive(Clone)]
pub(super) struct RowHandle {
    pub mounted: Option<Rc<MountedData>>,
    pub message_id: Option<String>,
    pub anchor: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Rendered {
    pub spacer_height: f64,
    pub scrollable: Scrollable,
    pub autoscrolling: bool,
    pub pending: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ScrollerContext {
    pub(super) scope: ScopeId,
    pub(super) controller: Signal<Controller>,
    pub(super) viewport: Signal<Option<Rc<MountedData>>>,
    pub(super) rows: Signal<BTreeMap<u64, RowHandle>>,
    pub(super) next_key: Signal<u64>,
    pub(super) rendered: Signal<Rendered>,
    pub(super) visibility: Signal<Visibility>,
    pub(super) watchers: Signal<usize>,
    pub(super) pending_move: Signal<Option<Move>>,
    pub(super) queued: Signal<Vec<Job>>,
    pub(super) padding_end: Signal<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Job {
    Content,
    Resize,
    Scroll,
}

impl ScrollerContext {
    pub(super) fn new(config: Config) -> Self {
        let controller = Controller::new(config);
        let rendered = Rendered {
            pending: controller.pending_default(),
            ..Rendered::default()
        };
        Self {
            scope: current_scope_id(),
            controller: Signal::new(controller),
            viewport: Signal::new(None),
            rows: Signal::new(BTreeMap::new()),
            next_key: Signal::new(0),
            rendered: Signal::new(rendered),
            visibility: Signal::new(Visibility::default()),
            watchers: Signal::new(0),
            pending_move: Signal::new(None),
            queued: Signal::new(Vec::new()),
            padding_end: Signal::new(0.0),
        }
    }

    pub(super) fn spawn(self, task: impl Future<Output = ()> + 'static) {
        Runtime::current().spawn(self.scope, task);
    }

    pub(super) fn allocate_key(mut self) -> u64 {
        let key = *self.next_key.peek();
        self.next_key.set(key + 1);
        key
    }

    pub(super) fn register_row(
        mut self,
        key: u64,
        mounted: Rc<MountedData>,
        message_id: Option<String>,
        anchor: bool,
    ) {
        self.rows.write().insert(
            key,
            RowHandle {
                mounted: Some(mounted),
                message_id,
                anchor,
            },
        );
        self.queue(Job::Content);
    }

    pub(super) fn update_row(mut self, key: u64, message_id: Option<String>, anchor: bool) {
        let changed = match self.rows.write().get_mut(&key) {
            Some(row) if row.message_id != message_id || row.anchor != anchor => {
                row.message_id = message_id;
                row.anchor = anchor;
                true
            }
            _ => false,
        };
        if changed {
            self.queue(Job::Content);
        }
    }

    pub(super) fn unregister_row(mut self, key: u64) {
        if self.rows.write().remove(&key).is_some() {
            self.queue(Job::Content);
        }
    }

    pub(super) fn queue(mut self, job: Job) {
        let idle = self.queued.peek().is_empty();
        if !self.queued.peek().contains(&job) {
            self.queued.write().push(job);
        }
        if idle {
            self.spawn(async move { self.drain().await });
        }
    }

    async fn drain(mut self) {
        loop {
            let Some(job) = self.queued.peek().first().copied() else {
                return;
            };
            self.run_job(job).await;
            self.queued.write().retain(|queued| *queued != job);
        }
    }

    async fn run_job(self, job: Job) {
        let Some(layout) = self.measure().await else {
            return;
        };
        let effects = match job {
            Job::Content => self
                .controller
                .clone()
                .write()
                .handle_content_change(&layout),
            Job::Resize => self.controller.clone().write().handle_resize(&layout),
            Job::Scroll => self.on_scroll(&layout),
        };
        self.finish(&layout, effects);
    }

    pub(super) fn update(
        self,
        operation: impl FnOnce(&mut Controller, &super::geometry::Layout, &mut Effects) + 'static,
    ) {
        self.spawn(async move {
            let Some(layout) = self.measure().await else {
                return;
            };
            let mut effects = Effects::default();
            operation(&mut self.controller.clone().write(), &layout, &mut effects);
            self.finish(&layout, effects);
        });
    }

    pub(super) fn finish(mut self, layout: &super::geometry::Layout, effects: Effects) {
        if effects.commit {
            self.controller.write().commit(layout);
        }
        self.publish();
        if effects.sync_visibility {
            self.sync_visibility(layout);
        }
        if effects.scroll.is_some() {
            self.pending_move.set(effects.scroll);
        }
    }

    pub(super) fn publish(mut self) {
        let controller = self.controller.peek();
        let next = Rendered {
            spacer_height: controller.spacer_height(),
            scrollable: controller.published(),
            autoscrolling: controller.autoscrolling(),
            pending: controller.pending_default(),
        };
        drop(controller);
        if *self.rendered.peek() != next {
            self.rendered.set(next);
        }
    }
}
