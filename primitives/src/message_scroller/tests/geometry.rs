use std::collections::HashSet;

use super::super::types::Align;
use super::{GAP, PADDING, VIEWPORT, key_of, layout, message, turn};

#[test]
fn content_bottom_is_the_last_row_plus_padding_whatever_the_scroll() {
    let at_top = layout(&[message("a", 300.0), message("b", 300.0)], 0.0);
    let scrolled = layout(&[message("a", 300.0), message("b", 300.0)], 120.0);
    let expected = PADDING + 300.0 + GAP + 300.0 + PADDING;
    assert_eq!(at_top.content_bottom(), expected);
    assert_eq!(scrolled.content_bottom(), expected);
    assert_eq!(at_top.max_scroll_top(), expected - VIEWPORT);
}

#[test]
fn scrollable_edges_respect_the_threshold() {
    let rows = [message("a", 300.0), message("b", 300.0)];
    let max = layout(&rows, 0.0).max_scroll_top();
    assert_eq!(
        (
            layout(&rows, 0.0).scrollable(8.0).start,
            layout(&rows, 0.0).scrollable(8.0).end
        ),
        (false, true)
    );
    assert!(!layout(&rows, 8.0).scrollable(8.0).start);
    assert!(layout(&rows, 9.0).scrollable(8.0).start);
    assert!(!layout(&rows, max - 8.0).scrollable(8.0).end);
    assert!(layout(&rows, max - 9.0).scrollable(8.0).end);
}

#[test]
fn a_short_transcript_scrolls_nowhere() {
    let short = layout(&[message("a", 50.0)], 0.0);
    assert_eq!(short.scrollable(8.0), Default::default());
    assert_eq!(short.max_scroll_top(), 0.0);
}

#[test]
fn element_scroll_top_aligns_start_center_end_and_nearest() {
    let rows = [
        message("a", 300.0),
        message("b", 100.0),
        message("c", 300.0),
    ];
    let at_top = layout(&rows, 0.0);
    let b = at_top.row(key_of("b")).unwrap().clone();
    let b_top = PADDING + 300.0 + GAP;
    assert_eq!(
        at_top.element_scroll_top(&b, Align::Start, 10.0),
        b_top - PADDING - 10.0
    );
    let inset = VIEWPORT - PADDING * 2.0;
    assert_eq!(
        at_top.element_scroll_top(&b, Align::Center, 0.0),
        b_top - PADDING - (inset - 100.0) / 2.0
    );
    assert_eq!(
        at_top.element_scroll_top(&b, Align::End, 0.0),
        b_top - VIEWPORT + 100.0 + PADDING
    );
    assert_eq!(
        at_top.element_scroll_top(&b, Align::Nearest, 0.0),
        b_top + 100.0 - VIEWPORT + PADDING
    );
    let showing = layout(&rows, 200.0);
    let b = showing.row(key_of("b")).unwrap().clone();
    assert_eq!(showing.element_scroll_top(&b, Align::Nearest, 0.0), 200.0);
}

#[test]
fn the_tail_spacer_fills_what_the_content_lacks_below_the_target() {
    let rows = [message("a", 100.0), turn("b", 60.0)];
    let at_top = layout(&rows, 0.0);
    let target = 150.0;
    let expected = (target + VIEWPORT - at_top.content_bottom() - GAP).ceil();
    assert_eq!(at_top.tail_spacer_height(target), expected);
    assert_eq!(at_top.tail_spacer_height(-500.0), 0.0);
}

#[test]
fn visibility_skips_the_peek_band_and_keeps_the_passed_anchor_current() {
    let rows = [
        turn("q1", 100.0),
        message("a1", 500.0),
        turn("q2", 100.0),
        message("a2", 100.0),
    ];
    let scrolled = layout(&rows, 300.0);
    let state = scrolled.visibility(0.0, 64.0);
    assert_eq!(
        state.visible_message_ids,
        vec!["a1".to_string(), "q2".to_string()]
    );
    assert_eq!(state.current_anchor_id.as_deref(), Some("q1"));
    let further = layout(&rows, 600.0);
    let state = further.visibility(0.0, 64.0);
    assert_eq!(state.current_anchor_id.as_deref(), Some("q2"));
    assert!(state.visible_message_ids.contains(&"a2".to_string()));
}

#[test]
fn anchors_are_found_among_new_rows_and_unhandled_rows() {
    let rows = [
        turn("q1", 50.0),
        message("a1", 50.0),
        turn("q2", 50.0),
        turn("q3", 50.0),
    ];
    let state = layout(&rows, 0.0);
    assert_eq!(
        state.new_scroll_anchor(2).map(|row| row.key),
        Some(key_of("q2"))
    );
    assert!(state.has_multiple_new_scroll_anchors(2));
    assert!(!state.has_multiple_new_scroll_anchors(3));
    assert_eq!(
        state.last_scroll_anchor().map(|row| row.key),
        Some(key_of("q3"))
    );
    let handled: HashSet<u64> = [key_of("q1"), key_of("q2")].into();
    assert_eq!(
        state.unhandled_scroll_anchor(&handled).map(|row| row.key),
        Some(key_of("q3"))
    );
}

#[test]
fn the_first_visible_row_needs_a_message_id() {
    let mut state = layout(&[message("a", 300.0), message("b", 300.0)], 320.0);
    assert_eq!(
        state.first_visible_row().map(|row| row.key),
        Some(key_of("b"))
    );
    state.rows[1].message_id = None;
    assert_eq!(state.first_visible_row(), None);
}
