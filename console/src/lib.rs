use leptos::*;
use leptos_router::*;

mod components;
mod pages;

use self::{components::*, pages::*};


#[component]
pub fn App(cx: Scope) -> impl IntoView {

    view! { cx,
        <Header />
        <Router>
            <main>
            </main>
        </Router>
    }
}
