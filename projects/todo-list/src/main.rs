use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            id: rand::rng().random_range(0..1000),
            description,
            completed: false,
        }
    }

    fn remove(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
        let index = tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or(format!("Task with id {} not found", id))?;
        tasks.remove(index);
        Ok(())
    }

    fn load(path: String) -> Result<Vec<Self>, std::io::Error> {
        let path = Path::new(&path);

        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }

        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    fn complete(tasks: &mut [Task], id: u32) -> Result<(), String> {
        let task = tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(println!("Error getting task {}", id))
            .unwrap();
        task.completed = true;
        Ok(())
    }

    fn save(tasks: &[Task], path: String) -> Result<(), std::io::Error> {
        let path = Path::new(&path);
        let json_string = serde_json::to_string_pretty(tasks)?;
        let mut file = File::create(path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }
}

fn parse_id(args: &[String], index: usize) -> Result<u32, String> {
    let id_str = args
        .get(index)
        .ok_or_else(|| format!("Index out of bounds: {}", index))?;
    id_str
        .parse()
        .map_err(|_| format!("Invalid ID: {}", id_str))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let file_path = "db/tasks.json".to_string();
    let mut tasks: Vec<Task> = Task::load(file_path).unwrap();

    match action.as_ref() {
        "add" => {
            let description: &String = &args[2];
            let task = Task::new(description.to_string());
            let file_path = "db/tasks.json".to_string();
            tasks.push(task);
            Task::save(&tasks, file_path).unwrap();
            println!("Task added and saved!");
        }
        "list" => {
            println!("{:?}", tasks);
        }
        "remove" => {
            let id = parse_id(&args, 2).unwrap();
            let file_path = "db/tasks.json".to_string();
            Task::remove(&mut tasks, id).unwrap();
            Task::save(&tasks, file_path).unwrap();
            println!("Task removed and saved!");
        }
        "clear" => {
            tasks.clear();
            let file_path = "db/tasks.json".to_string();
            Task::save(&tasks, file_path).unwrap();
            println!("Tasks cleared and saved!");
        }
        "done" => {
            let id = parse_id(&args, 2).unwrap();
            let file_path = "db/tasks.json".to_string();

            Task::complete(&mut tasks, id).unwrap();
            Task::save(&tasks, file_path).unwrap();
            println!("Task completed with id {}!", id);
        }
        _ => println!("Unknown action"),
    }
}
