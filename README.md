# ReMake: The Modern GNU Make Replacement

Welcome to ReMake, the GNU Make replacement designed for the 21st century.

## Overview

ReMake offers a more user-friendly and powerful alternative to GNU Make for developers. It utilizes a `tasks.toml` file to define tasks, allowing you to execute them using a familiar command syntax. Let's explore its features and usage.

## Usage

### Defining Tasks

Tasks are defined in the `tasks.toml` file using the following structure:

```toml
[task_name]
command = "command_to_execute"
```

For example, to define a build task that executes the `cargo build` command:

```toml
[build]
command = "cargo build"
```

### Running Tasks

To run a task, simply use the following command:

```
$ remake task_name
```

For instance, to execute the `build` task defined earlier:

```
$ remake build
```

### Controlling Output

You can control the output of a task by adding optional flags. For example, to suppress the standard output (stdout) for the `build` task, you can modify its definition as follows:

```toml
[build]
command = "cargo build"
quiet = true
```

### Handling Dependencies

ReMake supports handling dependencies between tasks. A dependency task is executed before the task it is associated with. Here's an example:

```toml
[build]
command = "cargo build"
dependencies = ["clean"]

[clean]
command = "cargo clean"
quiet = true
```

In this case, the `clean` task will be executed before the `build` task.

### Checking Targets

ReMake also allows you to specify targets, which are filepaths that should exist after a task has been executed. This feature helps ensure the success of a task. Here's an example:

```toml
[build]
command = "cargo build"
dependencies = ["clean"]
target = ["myprogram"]
```

If a `target` is defined for a task, ReMake will check for the specified file(s) after the task execution. If the file(s) cannot be found, ReMake will exit with a non-zero status.

## Installation

__Coming Soon__

Stay tuned for instructions on how to install ReMake.
