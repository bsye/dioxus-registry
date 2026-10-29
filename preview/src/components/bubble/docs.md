A chat bubble in seven variants (default, secondary, muted, tinted, outline, ghost, destructive), aligned to either side, with optional reactions. `BubbleContent` takes `r#as` to render as a button or link. A port of [shadcn/ui's Bubble](https://ui.shadcn.com/docs/components/bubble).

## Component Structure

```rust
BubbleGroup {
    Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::Start,
        BubbleContent { "Hello" }
        BubbleReactions { side: BubbleSide::Bottom, align: BubbleAlign::End, "👍" }
    }
}
```
