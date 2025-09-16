use dioxus::prelude::*;

use crate::models::PreviewState;

#[derive(PartialEq, Clone, Props)]
pub struct HomePageProps {
    story: ReadOnlySignal<PreviewState>,
}

#[component]
pub fn HomePage(props: HomePageProps) -> Element {
    rsx! {
    }
}
