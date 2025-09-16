use dioxus::prelude::*;

use crate::models::{Page, PreviewState, StoryItem};
use crate::services::HackerNews;
use crate::Route;

#[component]
pub fn StoryListing(story: ReadOnlySignal<i64>) -> Element {
    let story: Resource<dioxus::Result<Page>> = use_server_future(
        move || async move {
            Ok(HackerNews::get_page(story()).await?)
        }
    )?;

    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story().unwrap()?.item;

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if score == 1 { " point" } else { " points" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            div {
                font_size: "1.5rem",
                Link {
                    to: Route::HomePage {
                        story: PreviewState {
                            active_story: Some(id),
                        }
                    },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    "{score}"
                }
                div {
                    padding_left: "0.5rem",
                    "by {by}"
                }
                div {
                    padding_left: "0.5rem",
                    "{time}"
                }
                div {
                    padding_left: "0.5rem",
                    "{comments}"
                }
            }
        }
    }
}
