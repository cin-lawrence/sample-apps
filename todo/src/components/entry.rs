use dioxus::prelude::*;

use crate::models::todo::Todos;

#[derive(PartialEq, Clone, Props)]
pub struct EntryProps {
    todos: Signal<Todos>,
    id: u32,
}

#[allow(non_snake_case)]
pub fn Entry(mut props: EntryProps) -> Element {
    let mut editing = use_signal(|| false);
    let todo = &(props.todos.read())[&props.id];
    let is_checked = todo.checked;
    let is_completed = if is_checked { "completed" } else { "" };
    let is_editing = editing().then_some("editing").unwrap_or_default();

    rsx! {
        li {
            class: "{is_completed} {is_editing}",
            onclick: move |_| {
                if !is_checked {
                    editing.set(true)
                }
            },
            onfocusout: move |_| editing.set(false),
            div { class: "view",
                input { class: "toggle", r#type: "checkbox", id: "cbg-{todo.id}", checked: "{is_checked}",
                    onchange: move |evt| {
                        if let Some(todo) = props.todos.write().get_mut(&props.id) {
                            todo.checked = evt.value().parse().unwrap();
                        }
                    }
                }
                label { r#for: "cbg-{todo.id}", pointer_events: "none", "{todo.contents}" }
            }
            if editing() {
                input {
                    class: "edit",
                    value: "{todo.contents}",
                    oninput: move |evt| {
                        if let Some(todo) = props.todos.write().get_mut(&props.id) {
                            todo.contents = evt.value().clone();
                        }
                    },
                    autofocus: "true",
                    onkeydown: move |evt| {
                        match evt.key().to_string().as_str() {
                            "Enter" | "Escape" | "Tab" => editing.set(false),
                            _ => {}
                        }
                    },
                }
            }
        }
    }
}
