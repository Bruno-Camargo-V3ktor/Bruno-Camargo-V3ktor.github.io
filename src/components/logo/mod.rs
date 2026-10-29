use dioxus::prelude::*;
#[component]
pub fn Logo(large: bool) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/logo/styles.css") }
        svg { class: if large { "rose rose--large" } else { "rose" }, view_box: "0 0 100 100", role: "img", "aria-label": "Blue Rose Studio, rosa em asterisco",
            circle { cx: "50", cy: "50", r: "9" }
            for i in 0..8 {
                g { transform: "rotate({i * 45} 50 50)",
                    rect { class: "rose-petal petal-{i}", x: "44", y: "8", width: "12", height: "42", rx: "6" }
                }
            }
        }
    }
}
