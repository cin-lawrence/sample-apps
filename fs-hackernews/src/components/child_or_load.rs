use dioxus::prelude::*;

use crate::components::LoadingIndicator;

#[derive(PartialEq, Clone, Props)]
pub struct ChildrenOrLoadingProps {
    children: Element,
}

#[allow(non_snake_case)]
pub fn ChildrenOrLoading(props: ChildrenOrLoadingProps) -> Element {
    let children = props.children;

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
