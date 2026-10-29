A file or image attached to a message, with states (idle, uploading, processing, error, done), sizes (default, sm, xs), orientations and actions. A port of [shadcn/ui's Attachment](https://ui.shadcn.com/docs/components/attachment). Uses `button` from dioxus-components.

## Component Structure

```rust
AttachmentGroup {
    Attachment { state: AttachmentState::Uploading, size: AttachmentSize::Default, orientation: AttachmentOrientation::Horizontal,
        AttachmentMedia { variant: AttachmentMediaVariant::Icon, Spinner {} }
        AttachmentContent {
            AttachmentTitle { "report.pdf" }
            AttachmentDescription { "Uploading · 64%" }
        }
        AttachmentActions { AttachmentAction { aria_label: "Cancel upload", X {} } }
        AttachmentTrigger { aria_label: "Open report.pdf" }
    }
}
```
