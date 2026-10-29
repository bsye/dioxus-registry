use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

#[component]
pub fn DirectionProvider(
    #[props(into)] direction: ReadSignal<Direction>,
    children: Element,
) -> Element {
    use_context_provider(|| direction);
    rsx! {
        {children}
    }
}

pub fn use_direction() -> Direction {
    try_use_context::<ReadSignal<Direction>>()
        .map(|direction| direction())
        .unwrap_or_default()
}
