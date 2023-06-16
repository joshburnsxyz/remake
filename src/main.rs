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
        println!("Dependencies: {:?}\n", task.dependencies);
        println!("Targets: {:?}\n", task.targets);
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

    if let Some(task_arg) = matches.get_one::<String>("task") {
        if let Some(task) = tasks.get(task_arg) {
            // Execute task dependencies
            if let Some(dependencies) = &task.dependencies {
                for dependency in dependencies {
                    if let Some(dependency_task) = tasks.get(dependency) {
                        println!("Executing dependency task: {}", dependency);
                        dependency_task.execute();
                    } else {
                        println!("Dependency task '{}' not found.", dependency);
                    }
                }
            }

            // Execute called task
            println!("Executing task: {}", task_arg);
            task.execute();
            task.check_targets();
        } else {
            // Task not found
            println!("Task '{}' not found.", task_arg);
        }
    } else {
        list_available_tasks(tasks);
    }
}
