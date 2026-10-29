An input or textarea with addons, text and buttons around it, inline or above and below, like a search field or a chat composer. A port of [shadcn/ui's Input Group](https://ui.shadcn.com/docs/components/input-group). Uses `button` from dioxus-components.

## Component Structure

```rust
InputGroup {
    InputGroupTextarea { placeholder: "Ask anything" }
    InputGroupAddon { align: InputGroupAlign::BlockEnd,
        InputGroupText { "Enter to send" }
        InputGroupButton { size: ButtonSize::IconXs, aria_label: "Send", ArrowUp {} }
    }
}
```
