Tells components whether the text runs left to right or right to left. Set `dir` on the page as well; the provider only shares the direction with components that need it in code. A port of [shadcn/ui's Direction](https://ui.shadcn.com/docs/components/direction).

## Component Structure

```rust
DirectionProvider { direction: Direction::Rtl, {children} }
let direction = use_direction();
```
