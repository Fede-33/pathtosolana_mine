use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Priority {
    Alta,
    Media,
    Baja,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Subtask {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub done: bool,
    pub priority: Priority,
    pub tag: String,
    pub subtasks: Vec<Subtask>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Statistics {
    pub added_tasks: usize,
    pub done_tasks: usize,
    pub pending_tasks: usize,
    pub removed_tasks: usize,
}