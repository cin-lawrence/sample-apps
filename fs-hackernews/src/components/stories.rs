use dioxus::prelude::*;

use crate::components::{ChildrenOrLoading, StoryListing};
use crate::services::HackerNews;

#[component]
pub fn Stories() -> Element {
    let stories: Resource<dioxus::Result<Vec<i64>>> = use_server_future(
        move || async move {
            Ok(HackerNews::get_top_stories().await?)
        }
    )?;

    match stories().unwrap() {
        Ok(list) => rsx! {
            div {
                for story in list {
                    ChildrenOrLoading {
                        key: "{story}",
                        StoryListing { story }
                    }
                }
            }
        },
        Err(err) => rsx! {"An error occurred while fetching stories {err}"},
    }
}
