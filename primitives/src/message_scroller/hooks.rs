use dioxus::prelude::*;

use super::context::{Job, ScrollerContext};
use super::types::{Behavior, Config, ScrollDirection, ScrollOptions, Scrollable, Visibility};

#[derive(Clone, Copy, PartialEq)]
pub struct MessageScrollerCommands {
    context: ScrollerContext,
}

impl MessageScrollerCommands {
    pub fn scroll_to_end(self, options: ScrollOptions) {
        self.context.update(move |controller, layout, effects| {
            controller.scroll_to_end(layout, options, effects);
        });
    }

    pub fn scroll_to_start(self, options: ScrollOptions) {
        self.context.update(move |controller, layout, effects| {
            controller.scroll_to_start(layout, options, effects);
        });
    }

    pub fn scroll_to_message(self, message_id: impl Into<String>, options: ScrollOptions) {
        let message_id = message_id.into();
        self.context.update(move |controller, layout, effects| {
            controller.scroll_to_message(layout, &message_id, options, effects);
        });
    }

    pub fn scroll_toward(self, direction: ScrollDirection, behavior: Behavior) {
        let options = ScrollOptions::behavior(behavior);
        match direction {
            ScrollDirection::Start => self.scroll_to_start(options),
            ScrollDirection::End => self.scroll_to_end(options),
        }
    }
}

pub fn use_message_scroller() -> MessageScrollerCommands {
    MessageScrollerCommands {
        context: use_context::<ScrollerContext>(),
    }
}

pub fn use_message_scroller_scrollable() -> Memo<Scrollable> {
    let context = use_context::<ScrollerContext>();
    use_memo(move || context.rendered.read().scrollable)
}

pub fn use_message_scroller_visibility() -> ReadSignal<Visibility> {
    let mut context = use_context::<ScrollerContext>();
    use_hook(move || {
        let watchers = *context.watchers.peek();
        context.watchers.set(watchers + 1);
        context.queue(Job::Scroll);
    });
    use_drop(move || {
        let watchers = context.watchers.peek().saturating_sub(1);
        context.watchers.set(watchers);
        if watchers == 0 {
            context.visibility.set(Visibility::default());
        }
    });
    context.visibility.into()
}

impl ScrollerContext {
    pub(super) fn reconfigure(self, next: impl FnOnce(Config) -> Config + 'static) {
        let current = self.controller.peek().config();
        let next = next(current);
        if next != current {
            self.update(move |controller, layout, effects| {
                controller.set_config(next, layout, effects)
            });
        }
    }
}
