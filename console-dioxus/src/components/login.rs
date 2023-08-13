use dioxus::prelude::*;

#[inline_props]
pub fn Login(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full",
            h1 { "Test" }
        }
    })
}