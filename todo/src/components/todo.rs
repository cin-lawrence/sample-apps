use dioxus::prelude::*;

use std::collections::HashMap;

use crate::models::todo::{FilterStatus, TodoItem};
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::entry::Entry;

#[allow(non_snake_case)]
pub fn Todo() -> Element {
    let todos = use_signal(HashMap::<u32, TodoItem>::default);
    let status = use_signal(|| FilterStatus::All);
    let todo_id = use_signal(|| 0);

    let mut filtered_todos = todos()
        .iter()
        .filter(|(_, item)| match *status.read() {
            FilterStatus::All => true,
            FilterStatus::Active => !item.checked,
            FilterStatus::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    rsx! {
        div {
            Header {
                todos: todos,
                todo_id: todo_id,
            }
            ul { class: "todo-list",
                for id in filtered_todos.iter() {
                    Entry {
                        key: "{id}",
                        id: *id,
                        todos: todos
                    }
                }
            }
            if !todos.read().is_empty() {
                Footer {
                    status: status,
                    todos: todos,
                    items_left: filtered_todos.len(),
                }
            }
        }
    }
}
