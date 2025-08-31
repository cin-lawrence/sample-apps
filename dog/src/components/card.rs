#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    title: String,
    list: Vec<String>,
    onclick: EventHandler<>,
}

pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            onclick: move |_| props.onclick.call(()),
            class: "my-2 bg-gray-100 w-full rounded-lg p-4 hover:bg-gray-200 ease-in-out duration-75",
            h3 { class: "text-1xl", "{props.title}" }
            ul { class: "list-disc ml-8",
            }
        }
    }
}
