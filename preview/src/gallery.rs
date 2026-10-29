use dioxus::prelude::*;

use crate::components::{
    attachment, bubble, direction, input_group, marker, message, message_scroller, spinner,
};

struct Demo {
    name: &'static str,
    render: fn() -> Element,
}

const DEMOS: [Demo; 11] = [
    Demo {
        name: "Message",
        render: || rsx! { message::variants::main::Demo {} },
    },
    Demo {
        name: "Bubble",
        render: || rsx! { bubble::variants::main::Demo {} },
    },
    Demo {
        name: "Bubble variants",
        render: || rsx! { bubble::variants::styles::Demo {} },
    },
    Demo {
        name: "Marker",
        render: || rsx! { marker::variants::main::Demo {} },
    },
    Demo {
        name: "Attachment",
        render: || rsx! { attachment::variants::main::Demo {} },
    },
    Demo {
        name: "Attachment states",
        render: || rsx! { attachment::variants::states::Demo {} },
    },
    Demo {
        name: "Message scroller",
        render: || rsx! { message_scroller::variants::main::Demo {} },
    },
    Demo {
        name: "Input group",
        render: || rsx! { input_group::variants::main::Demo {} },
    },
    Demo {
        name: "Spinner",
        render: || rsx! { spinner::variants::main::Demo {} },
    },
    Demo {
        name: "Direction",
        render: || rsx! { direction::variants::main::Demo {} },
    },
    Demo {
        name: "Button",
        render: || rsx! { crate::components::button::Button { "Button" } },
    },
];

#[component]
pub fn Gallery() -> Element {
    rsx! {
        main { style: "display: grid; gap: 3rem; max-width: 40rem; margin: 0 auto; padding: 2rem 1rem; font-family: system-ui, sans-serif; background: var(--primary-color); color: var(--secondary-color-4);",
            h1 { "dioxus-registry" }
            for demo in DEMOS {
                section { key: "{demo.name}", style: "display: grid; gap: 1rem;",
                    h2 { style: "font-size: 1rem;", "{demo.name}" }
                    {(demo.render)()}
                }
            }
        }
    }
}
