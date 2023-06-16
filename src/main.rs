use std::path::Path;
use std::collections::HashMap;
use clap::{arg, value_parser, Command};

mod task;

const FILENAME: &str = "tasks.toml";

fn list_available_tasks(tasks: HashMap<String, task::Task>) {
     // Print the tasks
    for (task_name, task) in tasks {
        println!("Task: {}", task_name);
        println!("Command: {}", task.command);
        println!("Quiet: {}\n", task.quiet);
    }
}

fn main() {
    let matches = Command::new("remake")
        .arg(arg!([task] "Task to run").value_parser(value_parser!(String)))
        .get_matches();

    // Verify if the file exists
    if !Path::new(FILENAME).exists() {
        println!("Task file does not exist.");
        return;
    }

    let tasks = task::parse_taskfile(FILENAME);

    if let Some(task) = matches.get_one::<String>("task") {
        println!("Value for name: {task}");
    } else {
        list_available_tasks(tasks);
    }
}
