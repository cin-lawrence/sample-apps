use dioxus::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::{blog::Blog, home::Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
