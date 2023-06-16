use std::path::Path;

mod task;

fn main() {
    let file_path = "tasks.toml";

    // Verify if the file exists
    if !Path::new(file_path).exists() {
        println!("Task file does not exist.");
        return;
    }

    let tasks = task::parse_taskfile(file_path);

    // Print the tasks
    for (task_name, task) in tasks {
        println!("Task: {}", task_name);
        println!("Command: {}", task.command);
        println!("Quiet: {}\n", task.quiet);
    }
}
