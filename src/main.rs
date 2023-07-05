use std::io;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::process::{Command, exit};

mod task;

const FILENAME: &str = "tasks.toml";

fn list_available_tasks(tasks: &HashMap<String, task::Task>) {
    for (task_name, task) in tasks {
        println!("Task: {}", task_name);
        println!("Command: {}", task.command);
        println!("Quiet: {}\n", task.quiet);
        println!("Dependencies: {:?}\n", task.dependencies);
        println!("Targets: {:?}\n", task.targets);
    }
}

fn main() {
    if !std::path::Path::new(FILENAME).exists() {
        println!("Task file does not exist.");
        exit(1);
    }

    match task::parse_taskfile(FILENAME) {
        Ok(tasks) => {
            let matches = clap::App::new("remake")
                .arg(clap::Arg::new("task")
                    .about("Task to run")
                    .value_name("TASK")
                    .index(1))
                .get_matches();

            if let Some(task_arg) = matches.value_of("task") {
                match tasks.get(task_arg) {
                    Some(task) => {
                        if let Err(err) = task.execute() {
                            println!("Error executing task '{}': {}", task_arg, err);
                            exit(1);
                        }
                        task.check_targets();
                    }
                    None => {
                        println!("Task '{}' not found.", task_arg);
                        exit(1);
                    }
                }
            } else {
                list_available_tasks(&tasks);
            }
        }
        Err(err) => {
            println!("Error parsing task file: {}", err);
            exit(1);
        }
    }
}
