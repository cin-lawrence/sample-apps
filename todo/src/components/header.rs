use dioxus::prelude::*;

use crate::models::todo::{TodoItem, Todos};

#[derive(PartialEq, Clone, Props)]
pub struct HeaderProps {
    todos: Signal<Todos>,
    todo_id: Signal<u32>,
}

#[allow(non_snake_case)]
pub fn Header(props: HeaderProps) -> Element {
    let mut draft = use_signal(String::new);
    let mut todos = props.todos;
    let mut todo_id = props.todo_id;

    rsx! {
        header { class: "header",
            h1 {"todos"}
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{draft.read()}",
                autofocus: "true",
                oninput: move |evt| draft.set(evt.value().clone()),
                onkeydown: move |evt| {
                    if evt.key() == Key::Enter && !draft.read().is_empty() {
                        todos.write().insert(
                            todo_id(),
                            TodoItem {
                                id: todo_id(),
                                checked: false,
                                contents: draft.read().clone(),
                            },
                        );
                        todo_id.set(todo_id() + 1);
                        draft.set("".to_string());
                    }
                }
            }
        }
    }
}
