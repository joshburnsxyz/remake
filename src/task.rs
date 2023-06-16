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
