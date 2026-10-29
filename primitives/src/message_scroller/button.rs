use dioxus::prelude::*;

use super::hooks::{use_message_scroller, use_message_scroller_scrollable};
use super::types::{Behavior, ScrollDirection};

#[component]
pub fn MessageScrollerButton(
    #[props(default)] direction: ScrollDirection,
    #[props(default = Behavior::Smooth)] behavior: Behavior,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let commands = use_message_scroller();
    let active = use_message_scroller_scrollable()().toward(direction);
    rsx! {
        button {
            r#type: "button",
            "data-direction": direction.as_str(),
            "data-active": active,
            tabindex: if active { "0" } else { "-1" },
            aria_hidden: !active,
            onclick: move |_| commands.scroll_toward(direction, behavior),
            ..attributes,
            {children}
        }
    }
}
