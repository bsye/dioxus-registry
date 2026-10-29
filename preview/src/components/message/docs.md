Lays out one chat message: an avatar, an optional header and footer, and the content, aligned to the start or the end. A port of [shadcn/ui's Message](https://ui.shadcn.com/docs/components/message).

## Component Structure

```rust
MessageGroup {
    Message { align: MessageAlign::End,
        MessageAvatar { "ME" }
        MessageContent {
            MessageHeader { "You" }
            Bubble { BubbleContent { "Hello" } }
            MessageFooter { "Delivered" }
        }
    }
}
```
