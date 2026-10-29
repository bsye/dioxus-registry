use dioxus::prelude::*;

use super::super::parts::{
    MessageScroller, MessageScrollerContent, MessageScrollerItem, MessageScrollerProvider,
    MessageScrollerViewport,
};
use super::super::types::DefaultScrollPosition;

fn transcript(position: DefaultScrollPosition) -> String {
    let mut dom = VirtualDom::new_with_props(Transcript, TranscriptProps { position });
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[component]
fn Transcript(position: DefaultScrollPosition) -> Element {
    rsx! {
        MessageScrollerProvider { default_scroll_position: position,
            MessageScroller {
                MessageScrollerViewport {
                    MessageScrollerContent {
                        MessageScrollerItem { message_id: "m1", scroll_anchor: true, "Hello" }
                    }
                }
            }
        }
    }
}

#[test]
fn the_server_hides_a_transcript_that_opens_at_the_end_until_it_is_placed() {
    let html = transcript(DefaultScrollPosition::End);
    assert!(html.contains("data-pending-scroll"));
    assert!(html.contains("role=\"log\""));
    assert!(html.contains("aria-relevant=\"additions\""));
    assert!(html.contains("data-message-id=\"m1\""));
    assert!(html.contains("data-scroll-anchor=\"true\""));
}

#[test]
fn a_transcript_that_opens_at_the_start_shows_at_once() {
    assert!(!transcript(DefaultScrollPosition::Start).contains("data-pending-scroll"));
}
