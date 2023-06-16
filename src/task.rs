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
    let toml_content = fs::read_to_string(file_path).expect("Failed to read file.");

    let parsed_toml: toml::Value = toml::from_str(&toml_content).expect("Failed to parse TOML.");

    extract_tasks(&parsed_toml)
}

fn extract_tasks(parsed_toml: &toml::Value) -> HashMap<String, Task> {
    let mut tasks: HashMap<String, Task> = HashMap::new();

    if let toml::Value::Table(table) = parsed_toml {
        for (task_name, task_value) in table {
            let command = task_value["command"]
                .as_str()
                .expect("Invalid command.")
                .to_string();

            let quiet = task_value.get("quiet")
                .and_then(|quiet_value| quiet_value.as_bool())
                .unwrap_or(false);

            let task = Task { command, quiet };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}
