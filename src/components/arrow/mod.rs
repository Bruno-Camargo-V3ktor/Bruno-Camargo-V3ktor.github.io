use dioxus::prelude::*;
#[component]
pub fn Arrow() -> Element {
    rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", "aria-hidden": "true", path { d: "M5 19 19 5M5 5h14v14" } } }
}
