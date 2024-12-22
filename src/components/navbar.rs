use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::Login {}, "Login" }
                }
            }
        }
        Outlet::<Route> {}
    }
}