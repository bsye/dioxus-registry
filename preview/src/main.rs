use dioxus::prelude::*;

const THEME: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: THEME }
        preview::gallery::Gallery {}
    }
}
