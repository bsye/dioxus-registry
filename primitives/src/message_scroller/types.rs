use dioxus::prelude::ScrollBehavior;

pub const DEFAULT_SCROLL_EDGE_THRESHOLD: f64 = 8.0;
pub const DEFAULT_SCROLL_PREVIOUS_ITEM_PEEK: f64 = 64.0;
pub const DEFAULT_SCROLL_MARGIN: f64 = 0.0;
pub const USER_SCROLL_KEYS: [&str; 7] = [
    "ArrowDown",
    "ArrowUp",
    "End",
    "Home",
    "PageDown",
    "PageUp",
    " ",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    Nearest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Behavior {
    #[default]
    Auto,
    Smooth,
}

impl Behavior {
    pub fn native(self) -> ScrollBehavior {
        match self {
            Behavior::Auto => ScrollBehavior::Instant,
            Behavior::Smooth => ScrollBehavior::Smooth,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollOptions {
    pub align: Align,
    pub behavior: Behavior,
    pub scroll_margin: Option<f64>,
}

impl ScrollOptions {
    pub fn behavior(behavior: Behavior) -> Self {
        Self {
            behavior,
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DefaultScrollPosition {
    Start,
    #[default]
    End,
    LastAnchor,
}

impl DefaultScrollPosition {
    pub fn holds_viewport(self) -> bool {
        self != DefaultScrollPosition::Start
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollDirection {
    Start,
    #[default]
    End,
}

impl ScrollDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            ScrollDirection::Start => "start",
            ScrollDirection::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scrollable {
    pub start: bool,
    pub end: bool,
}

impl Scrollable {
    pub fn toward(self, direction: ScrollDirection) -> bool {
        match direction {
            ScrollDirection::Start => self.start,
            ScrollDirection::End => self.end,
        }
    }

    pub fn attribute(self) -> Option<&'static str> {
        match (self.start, self.end) {
            (true, true) => Some("start end"),
            (true, false) => Some("start"),
            (false, true) => Some("end"),
            (false, false) => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Visibility {
    pub current_anchor_id: Option<String>,
    pub visible_message_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub auto_scroll: bool,
    pub default_scroll_position: DefaultScrollPosition,
    pub scroll_edge_threshold: f64,
    pub scroll_previous_item_peek: f64,
    pub scroll_margin: f64,
    pub preserve_scroll_on_prepend: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_scroll: false,
            default_scroll_position: DefaultScrollPosition::End,
            scroll_edge_threshold: DEFAULT_SCROLL_EDGE_THRESHOLD,
            scroll_previous_item_peek: DEFAULT_SCROLL_PREVIOUS_ITEM_PEEK,
            scroll_margin: DEFAULT_SCROLL_MARGIN,
            preserve_scroll_on_prepend: true,
        }
    }
}
