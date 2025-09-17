use dioxus::prelude::*;

use crate::models::PreviewState;
use crate::components::{Preview, Stories};

const HACKERNEWS_CSS: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn HomePage(story: ReadOnlySignal<PreviewState>) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: HACKERNEWS_CSS,
        }
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: "50%",
                SuspenseBoundary {
                    fallback: |_context: SuspenseContext| rsx! {
                        "Loading..."
                    },
                    Stories {}
                }
            }
            div {
                width: "50%",
                SuspenseBoundary {
                    fallback: |_context: SuspenseContext| rsx! {
                        "Loading preview..."
                    },
                    Preview {
                        story
                    }
                }
            }
        }
    }
}
