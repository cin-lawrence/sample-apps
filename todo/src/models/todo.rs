use std::collections::HashMap;

#[derive(PartialEq)]
pub enum FilterStatus {
    All,
    Active,
    Completed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

pub type Todos = HashMap<u32, TodoItem>;
