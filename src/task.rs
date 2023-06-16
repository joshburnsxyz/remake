use std::io;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug)]
pub struct Task {
    pub command: String,
    pub quiet: bool,
}

impl Task {
    pub fn execute(&self) {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .expect("Failed to execute command.");

        if !self.quiet {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }
}

pub fn parse_taskfile(file_path: &str) -> HashMap<String, Task> {
    // Read the TOML file
    let toml_content = fs::read_to_string(file_path).expect("Failed to read file.");

    // Parse the TOML content
    let parsed_toml: toml::Value = toml::from_str(&toml_content).expect("Failed to parse TOML.");

    // Extract tasks from the TOML table
    let tasks = extract_tasks(&parsed_toml);

    tasks
}

fn extract_tasks(parsed_toml: &toml::Value) -> HashMap<String, Task> {
    // Create a HashMap to store the tasks
    let mut tasks: HashMap<String, Task> = HashMap::new();

    // Iterate over the TOML table entries
    if let toml::Value::Table(table) = parsed_toml {
        for (task_name, task_value) in table {
            // Parse command string
            let command = task_value["command"]
                .as_str()
                .expect("Invalid command.")
                .to_string();

             // Check if the quiet field is present in the TOML table entry
            let quiet = if let Some(quiet_value) = task_value.get("quiet") {
                // If the quiet field is present, attempt to parse its value as a boolean
                quiet_value
                    .as_bool()
                    .expect("Invalid quiet value. Expected boolean.")
            } else {
                // If the quiet field is not present, default to false
                false
            };

            // Build task and add to hashmap
            let task = Task { command, quiet };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}
