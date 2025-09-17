use dioxus::prelude::*;

use crate::models::Comment as CommentData;
use crate::components::ChildrenOrLoading;
use crate::services::HackerNews;

#[derive(PartialEq, Clone, Props)]
pub struct CommentProps {
    comment: i64,
}

#[allow(non_snake_case)]
pub fn Comment(props: CommentProps) -> Element {
    let comment: Resource<dioxus::Result<CommentData>> = use_server_future(
        use_reactive!(
            |props| async move {
                let comment = HackerNews::get_comment(props.comment).await?;
                Ok(comment)
            }
        )
    )?;

    let CommentData {
        by,
        text,
        kids,
        ..
    } = comment().unwrap()?;

    rsx! {
        div {
            padding: "0.5rem",
            div {
                color: "gray",
                "by {by}"
            }
            div {
                dangerous_inner_html: "{text}"
            }
            for comment in kids.iter().copied() {
                ChildrenOrLoading {
                    key: "{comment}",
                    Comment {
                        comment
                    }
                }
            }
        }
    }
}
