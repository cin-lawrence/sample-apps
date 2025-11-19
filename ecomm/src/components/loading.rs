use dioxus::prelude::*;

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
                match context.suspense_placeholder() {
                    Some(placeholder) => rsx! {
                        {placeholder}
                    },
                    None => rsx! {
                        LoadingIndicator {}
                    }
                }
            },
            children
        }
    }
}


#[allow(non_snake_case)]
pub fn LoadingIndicator() -> Element {
    rsx! {
        div {
            class: "spinner",
        }
    }
}
