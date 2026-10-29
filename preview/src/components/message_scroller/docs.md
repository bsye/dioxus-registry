A chat transcript that follows new messages while you are at the end, settles a new turn near the top with the previous message peeking above, keeps your place when older messages load, opens at the start, the end or the last turn, and shows a scroll button when there is more to see. A port of [shadcn/ui's Message Scroller](https://ui.shadcn.com/docs/components/message-scroller) and its headless `@shadcn/react/message-scroller`, whose behaviour lives in `dioxus-registry-primitives`.

## Component Structure

```rust
MessageScrollerProvider { auto_scroll: true, default_scroll_position: DefaultScrollPosition::End,
    MessageScroller {
        MessageScrollerViewport { preserve_scroll_on_prepend: true,
            MessageScrollerContent {
                MessageScrollerItem { message_id: "m1", scroll_anchor: true, {children} }
            }
        }
        MessageScrollerButton { direction: ScrollDirection::End }
    }
}
```

Hooks: `use_message_scroller()` (scroll to the start, the end or a message), `use_message_scroller_scrollable()` (which edges can still be scrolled to) and `use_message_scroller_visibility()` (the visible message ids and the current turn).
