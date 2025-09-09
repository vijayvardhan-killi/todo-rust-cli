use std::fs;
use std::path::Path;
use super::tasks::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Unable to serialize tasks");
    fs::write(FILE_PATH, data).expect("Unable to write file");
}
