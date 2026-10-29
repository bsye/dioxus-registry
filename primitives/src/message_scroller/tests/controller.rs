use super::super::controller::{Mode, Move};
use super::super::types::Behavior;
use super::{following, layout, message, opened};

#[test]
fn the_transcript_opens_at_the_end_and_releases_the_hidden_viewport() {
    let mut controller = following();
    assert!(controller.pending_default());
    let rows = [message("a", 300.0), message("b", 300.0)];
    let effects = controller.handle_content_change(&layout(&rows, 0.0));
    assert_eq!(
        effects.scroll,
        Some(Move {
            top: layout(&rows, 0.0).max_scroll_top(),
            behavior: Behavior::Auto
        })
    );
    assert!(!controller.pending_default());
    assert_eq!(controller.mode(), Mode::FollowingBottom);
}

#[test]
fn an_empty_transcript_shows_at_once() {
    let mut controller = following();
    let effects = controller.handle_content_change(&layout(&[], 0.0));
    assert_eq!(effects.scroll, None);
    assert!(!controller.pending_default());
}

#[test]
fn a_reader_at_the_end_follows_new_messages() {
    let mut controller = following();
    let top = opened(&mut controller, &[message("a", 300.0), message("b", 300.0)]);
    let grown = layout(
        &[
            message("a", 300.0),
            message("b", 300.0),
            message("c", 200.0),
        ],
        top,
    );
    let effects = controller.handle_content_change(&grown);
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(grown.max_scroll_top())
    );
}

#[test]
fn a_reader_who_scrolled_away_keeps_their_place_and_sees_the_end_button() {
    let mut controller = following();
    let rows = [message("a", 300.0), message("b", 300.0)];
    let top = opened(&mut controller, &rows);
    controller.on_scroll(&layout(&rows, top - 100.0), f64::MAX);
    assert_eq!(controller.mode(), Mode::FreeScrolling);
    let grown = layout(
        &[
            message("a", 300.0),
            message("b", 300.0),
            message("c", 200.0),
        ],
        top - 100.0,
    );
    let effects = controller.handle_content_change(&grown);
    assert_eq!(effects.scroll, None);
    controller.commit(&grown);
    assert!(controller.published().end);
}

#[test]
fn the_end_stays_hidden_while_following() {
    let mut controller = following();
    let rows = [message("a", 300.0), message("b", 300.0)];
    opened(&mut controller, &rows);
    controller.commit(&layout(&[message("a", 300.0), message("b", 600.0)], 240.0));
    assert!(!controller.published().end);
}
