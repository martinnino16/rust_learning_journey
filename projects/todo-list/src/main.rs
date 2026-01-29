use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use rand::RngExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main()  {
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let file_path = "db/tasks.json".to_string();
    let path = Path::new(&file_path);

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut tasks: Vec<Task> = Vec::new();
    let mut file = File::open(&path).unwrap();
    let mut contents: String = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    if contents.trim().is_empty() {
        tasks = Vec::new();
    } else {
        tasks = serde_json::from_str(&contents).unwrap();
    };
    
    match action.as_ref() {
        "add" => {
            let description: &String = &args[2];
            let task = Task {
                id: rand::rng().random_range(0..1000),
                description: description.to_string(),
                completed: false
            };
            tasks.push(task);

            let json_string = match serde_json::to_string_pretty(&tasks) {
                Err(why) => panic!("couldn't serialize task: {}", why),
                Ok(string) => string,
            };
            let mut file = File::create(path).unwrap();

            match file.write_all(json_string.as_bytes()) {
                Err(why) => panic!("couldn't write task: {}", why),
                Ok(_) => println!("task saved"),
            };
        }
        "list" => {
            println!("{:?}", tasks);
        }
        "remove" => {
            let id = &args[2];
            let id_str: u32 = id.trim().parse().expect("id is not a number!");
            let index = tasks.iter().position(|task| task.id == id_str).unwrap();
            tasks.remove(index);
            let mut file = File::create(path).unwrap();
            let json_string = match serde_json::to_string_pretty(&tasks) {
                Err(why) => panic!("couldn't serialize task: {}", why),
                Ok(string) => string,
            };
            let mut file= File::create(path).unwrap();
            match file.write_all(json_string.as_bytes()) {
                Err(why) => panic!("couldn't write task: {}", why),
                Ok(_) => println!("task deleted"),
            }
        }
        "clear" => {
            tasks.clear();
            let json_string = match serde_json::to_string_pretty(&tasks) {
                Err(why) => panic!("couldn't serialize task: {}", why),
                Ok(string) => string,
            };
            let mut file = File::create(path).unwrap();
            match file.write_all(json_string.as_bytes()) {
                Err(why) => panic!("couldn't write task: {}", why),
                Ok(_) => println!("tasks cleared"),
            }
        }
        "done" => {
            let id = &args[2];
            let id_str: u32 = id.trim().parse().expect("id is not a number!");
            let task = match tasks.iter_mut().find(|task| task.id == id_str ).ok_or(println!("Error getting task {}", id_str )) {
                Ok(task) => task,
                Err(e) => panic!("Error getting task")
            };
            task.completed = true;
            let json_string = match serde_json::to_string_pretty(&tasks) {
                Err(why) => panic!("couldn't serialize task: {}", why),
                Ok(string) => string,
            };
            let mut file = File::create(path).unwrap();
            match file.write_all(json_string.as_bytes()) {
                Err(why) => panic!("couldn't write task: {}", why),
                Ok(_) => println!("tasks updated"),
            }

        }
        _ => println!("Unknown action")
    }
}
