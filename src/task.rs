use std::io;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub command: String,
    pub quiet: bool,
    pub dependencies: Option<Vec<String>>,
    pub target: Option<String>,
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
    let toml_content = fs::read_to_string(file_path).expect("Failed to read file.");

    let parsed_toml: toml::Value = toml::from_str(&toml_content).expect("Failed to parse TOML.");

    extract_tasks(&parsed_toml)
}

fn extract_tasks(parsed_toml: &toml::Value) -> HashMap<String, Task> {
    let mut tasks: HashMap<String, Task> = HashMap::new();

    if let toml::Value::Table(table) = parsed_toml {
        for (task_name, task_value) in table {
            // Parse command to run
            let command = task_value["command"]
                .as_str()
                .expect("Invalid command.")
                .to_string();

            // Parse the quiet flag
            let quiet = task_value.get("quiet")
                .and_then(|quiet_value| quiet_value.as_bool())
                .unwrap_or(false);

            // Parse dependency tasks
            let dependencies = task_value.get("dependencies")
                .and_then(|dependencies_value| dependencies_value.as_array())
                .map(|dependencies|{
                    dependencies
                    .iter()
                    .filter_map(|dependency_value| dependency_value.as_str())
                    .map(|dependency| dependency.to_string())
                    .collect::<Vec<String>>()
                });

            let target = task_value.get("target")
                .and_then(|target_value| Some(target_value.to_string()));

            let task = Task { command, quiet, dependencies, target };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}
