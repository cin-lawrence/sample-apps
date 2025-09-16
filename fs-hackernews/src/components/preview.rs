use dioxus::prelude::*;

use crate::components::{ChildrenOrLoading, Comment};
use crate::models::{Page, PreviewState};
use crate::services::HackerNews;

#[derive(PartialEq, Clone, Props)]
pub struct PreviewProps {
    pub story: ReadOnlySignal<PreviewState>,
}

#[allow(non_snake_case)]
pub fn Preview(props: PreviewProps) -> Element {
    let PreviewState {
        active_story: Some(id),
    } = (props.story)()
    else {
        return rsx! {"Hover over a story to preview it here"};
    };

    let story: Resource<dioxus::Result<Page>> = use_server_future(
        use_reactive!(
            |id| async move {
                let page = HackerNews::get_page(id).await?;
                Ok(page)
            }
        )
    )?;
    let story = story().unwrap()?;

    rsx! {
        div {
            padding: "0.5rem",
            div {
                font_size: "1.5rem",
                a {
                    href: story.item.url,
                    "{story.item.title}"
                }
            }
            if let Some(text) = &story.item.text {
                div {
                    dangerous_inner_html: "{text}"
                }
            }
            for comment in story.item.kids.iter().copied() {
                ChildrenOrLoading {
                    key: "{comment}",
                    Comment { comment }
                }
            }
        }
    }
}
