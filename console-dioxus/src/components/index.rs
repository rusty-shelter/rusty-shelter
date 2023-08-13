use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full",
            h1 { "Homepage" }
            Link {
                to: "/login",
                "Go test!"
            }
        }
    })
}