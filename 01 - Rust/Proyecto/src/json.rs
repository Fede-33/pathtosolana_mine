use crate::data::*;
use std::fs;

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Vec::new()
        }),
        Err(_) => {
            Vec::new()
        }
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let serialized = serde_json::to_string_pretty(tasks).expect("Error al serializar las tareas.");
    fs::write("tasks.json", serialized).expect("Error al escribir el archivo.");
}

pub fn load_tags() -> Vec<String> {
    match fs::read_to_string("tags.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Vec::new()
        }),
        Err(_) => {
            Vec::new()
        }
    }
}

pub fn save_tags(tags: &Vec<String>) {
    let serialized = serde_json::to_string_pretty(tags).expect("Error al serializar los tags.");
    fs::write("tags.json", serialized).expect("Error al escribir el archivo.");
}

pub fn load_stats() -> Statistics {
    match fs::read_to_string("stats.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Statistics::default()
        }),
        Err(_) => {
            Statistics::default()
        }
    }
}

pub fn save_stats(stats: &Statistics) {
    let serialized = serde_json::to_string_pretty(stats).expect("Error al serializar las estadísticas.");
    fs::write("stats.json", serialized).expect("Error al escribir el archivo.");
}