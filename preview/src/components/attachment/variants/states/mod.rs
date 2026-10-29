use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleCheck, FileText, TriangleAlert};

use super::super::*;
use crate::components::spinner::Spinner;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 0.75rem;",
            Attachment { state: AttachmentState::Idle, style: "width: 100%;",
                AttachmentMedia { FileText {} }
                AttachmentContent {
                    AttachmentTitle { "Drop a file here" }
                    AttachmentDescription { "PDF, PNG or JPG" }
                }
            }
            Attachment { state: AttachmentState::Processing, size: AttachmentSize::Sm, style: "width: 100%;",
                AttachmentMedia { Spinner {} }
                AttachmentContent {
                    AttachmentTitle { "report.pdf" }
                    AttachmentDescription { "Reading the pages" }
                }
            }
            Attachment { state: AttachmentState::Error, size: AttachmentSize::Sm, style: "width: 100%;",
                AttachmentMedia { TriangleAlert {} }
                AttachmentContent {
                    AttachmentTitle { "archive.zip" }
                    AttachmentDescription { "Too large: the limit is 10 MB" }
                }
            }
            Attachment { size: AttachmentSize::Xs,
                AttachmentMedia { CircleCheck {} }
                AttachmentContent { AttachmentTitle { "notes.txt" } }
            }
        }
    }
}
