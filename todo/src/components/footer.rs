use dioxus::prelude::*;
use std::collections::HashMap;

use crate::models::todo::{FilterStatus, TodoItem};

#[derive(PartialEq, Clone, Props)]
pub struct FooterProps {
    status: Signal<FilterStatus>,
    todos: Signal<HashMap<u32, TodoItem>>,
    items_left: usize,
}

#[allow(non_snake_case)]
pub fn Footer(props: FooterProps) -> Element {
    let mut status = props.status;
    let mut todos = props.todos;
    let items_left = props.items_left;

    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };
    let show_clear_completed = todos().values().any(|todo| todo.checked);

    rsx! {
        footer { class: "footer",
            span { class: "todo-count",
                strong {"{items_left} "}
                span {"{item_text} left"}
            }
            ul { class: "filters",
                li { class: "All", a { onclick: move |_| status.set(FilterStatus::All), "All" }}
                li { class: "Active", a { onclick: move |_| status.set(FilterStatus::Active), "Active" }}
                li { class: "Completed", a { onclick: move |_| status.set(FilterStatus::Completed), "Completed" }}
            }
            if show_clear_completed {
                button {
                    class: "clear-completed",
                    onclick: move |_| todos.write().retain(|_, todo| !todo.checked),
                    "Clear completed"
                }
            }
        }
    }
}
