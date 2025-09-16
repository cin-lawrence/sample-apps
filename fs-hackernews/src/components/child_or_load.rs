use dioxus::prelude::*;

use crate::components::LoadingIndicator;

#[component]
pub fn ChildrenOrLoading(children: Element) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |context: SuspenseContext| {
                rsx! {
                    if let Some(placeholder) = context.suspense_placeholder() {
                        { placeholder }
                    } else {
                        LoadingIndicator {}
                    }
                }
            },
            children
        }
    }
}
