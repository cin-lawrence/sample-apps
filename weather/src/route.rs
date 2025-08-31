use dioxus::prelude::*;

use crate::components::home::Home;
use crate::components::navbar::Navbar;
use crate::components::weather::Weather;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/weather")]
    Weather {},
}
