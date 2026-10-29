use super::super::controller::{Controller, Mode};
use super::super::types::{Config, DefaultScrollPosition, ScrollOptions};
use super::{GAP, PADDING, VIEWPORT, following, layout, message, opened, turn};

#[test]
fn a_new_turn_settles_near_the_top_with_a_peek_and_a_spacer() {
    let mut controller = following();
    let top = opened(&mut controller, &[message("a", 300.0), message("b", 300.0)]);
    let grown = layout(
        &[message("a", 300.0), message("b", 300.0), turn("q", 40.0)],
        top,
    );
    let effects = controller.handle_content_change(&grown);
    let question = grown.rows[2].clone();
    let expected = grown.element_top(&question) - PADDING - 64.0;
    assert_eq!(effects.scroll.map(|next| next.top), Some(expected));
    assert_eq!(controller.mode(), Mode::AnchoredToMessage);
    assert_eq!(
        controller.spacer_height(),
        (expected + VIEWPORT - grown.content_bottom() - GAP).ceil()
    );
}

#[test]
fn several_turns_at_once_keep_following_the_end() {
    let mut controller = following();
    let top = opened(&mut controller, &[message("a", 300.0), message("b", 300.0)]);
    let rows = [
        message("a", 300.0),
        message("b", 300.0),
        turn("q1", 40.0),
        turn("q2", 40.0),
    ];
    let effects = controller.handle_content_change(&layout(&rows, top));
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(layout(&rows, top).max_scroll_top())
    );
    assert_eq!(controller.mode(), Mode::FollowingBottom);
}

#[test]
fn a_reply_that_fills_the_spacer_hands_back_to_following() {
    let mut controller = following();
    let top = opened(&mut controller, &[message("a", 300.0), message("b", 300.0)]);
    let asked = layout(
        &[message("a", 300.0), message("b", 300.0), turn("q", 40.0)],
        top,
    );
    let target = controller.handle_content_change(&asked).scroll.unwrap().top;
    assert!(controller.spacer_height() > 0.0);
    let answered = layout(
        &[
            message("a", 300.0),
            message("b", 300.0),
            turn("q", 40.0),
            message("r", 900.0),
        ],
        target,
    );
    controller.handle_content_change(&answered);
    let effects = controller.handle_resize(&answered);
    assert_eq!(controller.spacer_height(), 0.0);
    assert_eq!(controller.mode(), Mode::FollowingBottom);
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(answered.max_scroll_top())
    );
}

#[test]
fn prepended_history_keeps_the_first_visible_message_in_place() {
    let mut controller = Controller::new(Config::default());
    let rows = [message("a", 300.0), message("b", 300.0)];
    opened(&mut controller, &rows);
    controller.on_scroll(&layout(&rows, 100.0), f64::MAX);
    let prepended = layout(
        &[
            message("old", 200.0),
            message("a", 300.0),
            message("b", 300.0),
        ],
        100.0,
    );
    let effects = controller.handle_content_change(&prepended);
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(100.0 + 200.0 + GAP)
    );
}

#[test]
fn a_jump_to_a_message_not_loaded_yet_waits_for_it() {
    let mut controller = Controller::new(Config::default());
    let mut effects = Default::default();
    assert!(controller.scroll_to_message(
        &layout(&[], 0.0),
        "b",
        ScrollOptions::default(),
        &mut effects
    ));
    assert!(!controller.pending_default());
    let rows = layout(
        &[
            message("a", 300.0),
            message("b", 300.0),
            message("c", 300.0),
        ],
        0.0,
    );
    let effects = controller.handle_content_change(&rows);
    let b = rows.rows[1].clone();
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(rows.element_top(&b) - PADDING)
    );
    assert_eq!(controller.mode(), Mode::SettlingJump);
}

#[test]
fn last_anchor_opens_on_a_long_last_turn_and_at_the_end_for_a_short_one() {
    let config = Config {
        default_scroll_position: DefaultScrollPosition::LastAnchor,
        ..Config::default()
    };
    let long = layout(
        &[message("a", 300.0), turn("q", 40.0), message("r", 600.0)],
        0.0,
    );
    let mut controller = Controller::new(config);
    controller.handle_content_change(&long);
    assert_eq!(controller.mode(), Mode::AnchoredToMessage);
    let short = layout(
        &[message("a", 300.0), turn("q", 40.0), message("r", 60.0)],
        0.0,
    );
    let mut controller = Controller::new(config);
    let effects = controller.handle_content_change(&short);
    assert_eq!(
        effects.scroll.map(|next| next.top),
        Some(short.max_scroll_top())
    );
}

#[test]
fn a_wheel_gesture_releases_follow_and_anchoring() {
    let mut controller = following();
    opened(&mut controller, &[message("a", 300.0), message("b", 300.0)]);
    controller.user_intent();
    assert_eq!(controller.mode(), Mode::FreeScrolling);
}
