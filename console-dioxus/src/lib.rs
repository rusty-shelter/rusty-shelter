#![allow(non_snake_case, non_upper_case_globals)]

extern crate dioxus_router;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod components {
    pub mod index;
    pub use index::Index;
    pub mod login;
    pub use login::Login;
}

pub use components::*;

// 
fn DefaultLayout(cx: Scope) -> Element {
    render! {
        header {

        }
        Outlet::<Route> {}
        footer {
            
        }
    }
}

#[derive(Routable, Clone)]
enum Route {
  #[layout(DefaultLayout)]
    #[route("/")]
    Index {},

    #[route("/login")]
    Login {},
}

pub fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
