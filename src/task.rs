use std::io;
use std::io::Write;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub command: String,
    pub quiet: bool,
    pub dependencies: Option<Vec<String>>,
    pub targets: Option<Vec<String>>,
}

impl Task {
    pub fn execute(&self) -> io::Result<()> {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(&self.command)
            .output()?;

        if !self.quiet {
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
        }

        Ok(())
    }

    pub fn check_targets(&self) {
        if let Some(targets) = &self.targets {
            for target in targets {
                println!("Checking target: {}", target);
            }
        }
    }
}

pub fn parse_taskfile(file_path: &str) -> Result<HashMap<String, Task>, io::Error> {
    let toml_content = fs::read_to_string(file_path)?;

    let parsed_toml: toml::Value = toml::from_str(&toml_content)?;

    Ok(extract_tasks(&parsed_toml))
}

fn extract_tasks(parsed_toml: &toml::Value) -> HashMap<String, Task> {
    let mut tasks: HashMap<String, Task> = HashMap::new();

    if let toml::Value::Table(table) = parsed_toml {
        for (task_name, task_value) in table {
            let command = match task_value.get("command") {
                Some(value) => match value.as_str() {
                    Some(cmd) => cmd.to_string(),
                    None => {
                        eprintln!("Invalid command for task '{}'", task_name);
                        continue;
                    }
                },
                None => {
                    eprintln!("Missing command for task '{}'", task_name);
                    continue;
                }
            };

            let quiet = task_value
                .get("quiet")
                .and_then(|quiet_value| quiet_value.as_bool())
                .unwrap_or(false);

            let dependencies = task_value
                .get("dependencies")
                .and_then(|dependencies_value| dependencies_value.as_array())
                .map(|dependencies| {
                    dependencies
                        .iter()
                        .filter_map(|dependency_value| dependency_value.as_str())
                        .map(|dependency| dependency.to_string())
                        .collect::<Vec<String>>()
                });

            let targets = task_value
                .get("targets")
                .and_then(|targets_value| targets_value.as_array())
                .map(|targets| {
                    targets
                        .iter()
                        .filter_map(|target_value| target_value.as_str())
                        .map(|target| target.to_string())
                        .collect::<Vec<String>>()
                });

            let task = Task {
                command,
                quiet,
                dependencies,
                targets,
            };
            tasks.insert(task_name.to_string(), task);
        }
    }

    tasks
}
