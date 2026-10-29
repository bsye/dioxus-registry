A status line or separator inside a transcript: a branch switch, a search, "Thinking..." with a shimmer, or a divider. A port of [shadcn/ui's Marker](https://ui.shadcn.com/docs/components/marker).

## Component Structure

```rust
Marker { variant: MarkerVariant::Separator,
    MarkerIcon { Search {} }
    MarkerContent { shimmer: false, "Conversation compacted" }
}
```
