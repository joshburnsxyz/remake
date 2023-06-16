use std::io;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::process::{Command, exit};
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub command: String,
    pub quiet: bool,
    pub dependencies: Option<Vec<String>>,
    pub targets: Option<Vec<String>>,
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

    pub fn check_targets(&self) {
            if let Some(targets) = &self.targets {
                for target in targets {
                    println!("Checking target: {}", target);
                }
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

            // Parse dependency tasks
            let targets = task_value.get("targets")
                .and_then(|targets_value| targets_value.as_array())
                .map(|targets|{
                    targets
                    .iter()
                    .filter_map(|target_value| target_value.as_str())
                    .map(|target| target.to_string())
                    .collect::<Vec<String>>()
                });

            let task = Task { command, quiet, dependencies, targets };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}
