use dioxus::prelude::*;

use super::super::component::*;

const VARIANTS: [(BubbleVariant, &str); 7] = [
    (BubbleVariant::Default, "Default"),
    (BubbleVariant::Secondary, "Secondary"),
    (BubbleVariant::Muted, "Muted"),
    (BubbleVariant::Tinted, "Tinted"),
    (BubbleVariant::Outline, "Outline"),
    (BubbleVariant::Ghost, "Ghost"),
    (BubbleVariant::Destructive, "Destructive"),
];

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 0.75rem;",
            for (variant, label) in VARIANTS {
                Bubble { key: "{label}", variant, BubbleContent { "{label} bubble" } }
            }
            Bubble { variant: BubbleVariant::Outline, align: BubbleAlign::End,
                BubbleContent {
                    r#as: move |attributes: Vec<Attribute>| rsx! {
                        button { ..attributes, "A bubble that is a button" }
                    },
                }
            }
        }
    }
}
