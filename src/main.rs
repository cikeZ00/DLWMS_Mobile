mod services;
mod components;

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use components::navbar::NavBar;
use components::home::Home;
use components::login::Login;
use components::page_not_found::PageNotFound;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn app() -> Element {
    rsx! { Router::<Route> {} }
}

fn main() {
    dioxus::logger::initialize_default();
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(app)
}