use std::collections::HashMap;
use std::fs;
use std::path::Path;

use toml::Value;

mod task;

use task::Task;

fn parse_taskfile(file_path: &str) -> HashMap<String, Task> {
    // Read the TOML file
    let toml_content = fs::read_to_string(file_path).expect("Failed to read file.");

    // Parse the TOML content
    let parsed_toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML.");

    // Create a HashMap to store the tasks
    let mut tasks: HashMap<String, Task> = HashMap::new();

    // Iterate over the TOML table entries
    if let Value::Table(table) = parsed_toml {
        for (task_name, task_value) in table {
            // Parse command string
            let command = task_value["command"]
                .as_str()
                .expect("Invalid command.")
                .to_string();

            // Parse quiet settings true/false (default to false)
            let quiet = task_value["quiet"]
                .as_bool()
                .unwrap_or(false); // Assigns false as default if not provided

            // Build task and add to hashmap
            let task = Task { command, quiet };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}

fn main() {
    let file_path = "tasks.toml";

    // Verify if the file exists
    if !Path::new(file_path).exists() {
        println!("Task file does not exist.");
        return;
    }

    let tasks = parse_taskfile(file_path);

    // Print the tasks
    for (task_name, task) in tasks {
        println!("Task: {}", task_name);
        println!("Command: {}", task.command);
        println!("Quiet: {}\n", task.quiet);
    }
}
