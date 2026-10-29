use dioxus::prelude::*;

use super::context::{Job, ScrollerContext};
use super::types::{
    Config, DEFAULT_SCROLL_EDGE_THRESHOLD, DEFAULT_SCROLL_MARGIN,
    DEFAULT_SCROLL_PREVIOUS_ITEM_PEEK, DefaultScrollPosition, USER_SCROLL_KEYS,
};

#[component]
pub fn MessageScrollerProvider(
    #[props(default)] auto_scroll: bool,
    #[props(default)] default_scroll_position: DefaultScrollPosition,
    #[props(default = DEFAULT_SCROLL_EDGE_THRESHOLD)] scroll_edge_threshold: f64,
    #[props(default = DEFAULT_SCROLL_PREVIOUS_ITEM_PEEK)] scroll_previous_item_peek: f64,
    #[props(default = DEFAULT_SCROLL_MARGIN)] scroll_margin: f64,
    children: Element,
) -> Element {
    let config = Config {
        auto_scroll,
        default_scroll_position,
        scroll_edge_threshold,
        scroll_previous_item_peek,
        scroll_margin,
        ..Config::default()
    };
    let context = use_context_provider(|| ScrollerContext::new(config));
    use_effect(use_reactive!(|config| context.reconfigure(
        move |current| Config {
            preserve_scroll_on_prepend: current.preserve_scroll_on_prepend,
            ..config
        }
    )));
    rsx! {
        {children}
    }
}

#[component]
pub fn MessageScroller(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ScrollerContext>();
    let rendered = *context.rendered.read();
    rsx! {
        div {
            "data-scrollable": rendered.scrollable.attribute(),
            "data-autoscrolling": rendered.autoscrolling.then_some(""),
            "data-pending-scroll": rendered.pending.then_some(""),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MessageScrollerViewport(
    #[props(default = true)] preserve_scroll_on_prepend: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut context = use_context::<ScrollerContext>();
    use_effect(use_reactive!(|preserve_scroll_on_prepend| context
        .reconfigure(move |current| Config {
            preserve_scroll_on_prepend,
            ..current
        })));
    use_effect(move || {
        if context.pending_move.read().is_some() {
            context.spawn(async move {
                let next = context.pending_move.write().take();
                if let Some(next) = next {
                    context.perform(next).await;
                }
            });
        }
    });
    let rendered = *context.rendered.read();
    rsx! {
        div {
            role: "region",
            aria_label: "Messages",
            tabindex: "0",
            "data-scrollable": rendered.scrollable.attribute(),
            "data-autoscrolling": rendered.autoscrolling.then_some(""),
            "data-pending-scroll": rendered.pending.then_some(""),
            onmounted: move |event| {
                context.viewport.set(Some(event.data()));
                context.queue(Job::Content);
            },
            onscroll: move |_| context.queue(Job::Scroll),
            onresize: move |_| context.queue(Job::Resize),
            onwheel: move |_| context.controller.write().user_intent(),
            ontouchmove: move |_| context.controller.write().user_intent(),
            onkeydown: move |event| {
                if USER_SCROLL_KEYS.contains(&event.key().to_string().as_str()) {
                    context.controller.write().user_intent();
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MessageScrollerContent(
    spacer_class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ScrollerContext>();
    let spacer_height = context.rendered.read().spacer_height;
    rsx! {
        div {
            role: "log",
            aria_relevant: "additions",
            ..attributes,
            {children}
            div {
                aria_hidden: "true",
                "data-message-scroller-spacer": "",
                class: spacer_class,
                hidden: spacer_height == 0.0,
                style: "height: {spacer_height}px",
            }
        }
    }
}

#[component]
pub fn MessageScrollerItem(
    message_id: Option<String>,
    #[props(default)] scroll_anchor: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ScrollerContext>();
    let key = use_hook(|| context.allocate_key());
    use_effect(use_reactive!(
        |message_id, scroll_anchor| context.update_row(key, message_id, scroll_anchor)
    ));
    use_drop(move || context.unregister_row(key));
    let registered = message_id.clone();
    rsx! {
        div {
            "data-message-id": message_id,
            "data-scroll-anchor": scroll_anchor.then_some("true"),
            onmounted: move |event| context.register_row(key, event.data(), registered.clone(), scroll_anchor),
            onresize: move |_| context.queue(Job::Resize),
            ..attributes,
            {children}
        }
    }
}
