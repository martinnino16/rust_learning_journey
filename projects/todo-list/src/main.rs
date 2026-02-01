use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fmt, fs};
use std::fmt::Formatter;

const FILE_PATH: &str = "db/tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String, existing_tasks:&[Task]) -> Self {
        let max_id = existing_tasks.iter().map(|t| t.id).max().unwrap_or(0);
        Task {
            id: max_id + 1,
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

        if !path.exists() {
            return Ok(Vec::new());
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
            .ok_or_else(|| format!("Task with id {} not found", id))?;
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✅" } else { "⬜" };
        write!(f, "{} [{}] {}", status, self.id, self.description)
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



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No action provided");
        std::process::exit(1);
    }
    let action: &String = &args[1];

    let mut tasks: Vec<Task> = match Task::load(FILE_PATH.to_string()) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Warning: could not load tasks: {}", e);
            Vec::new()
        }
    };

    let result: Result<(), String> = match action.as_ref() {
        "add" => {
            let description: &String = &args[2];
            let task = Task::new(description.to_string(), &tasks);
            tasks.push(task);
            Task::save(&tasks, FILE_PATH.to_string()).map_err(|e| format!("Failed to save: {}", e))?;
            println!("Task added and saved!");
            Ok(())
        }
        "list" => {
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                println!("Tasks:");
                for task in &tasks {
                    println!("{}", task);
                }
                println!();
            }
            Ok(())
        }
        "remove" => {
            let id = parse_id(&args, 2)?;
            Task::remove(&mut tasks, id)?;
            Task::save(&tasks, FILE_PATH.to_string())?;
            println!("Task removed and saved!");
            Ok(())
        }
        "clear" => {
            tasks.clear();
            Task::save(&tasks, FILE_PATH.to_string())?;
            println!("Tasks cleared and saved!");
            Ok(())
        }
        "done" => {
            let id = parse_id(&args, 2)?;

            Task::complete(&mut tasks, id)?;
            Task::save(&tasks, FILE_PATH.to_string())?      ;
            println!("Task completed with id {}!", id);
            Ok(())
        }
        _ => {
            eprintln!("Error: Unknown action: {}", action);
            std::process::exit(1);
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())

}
