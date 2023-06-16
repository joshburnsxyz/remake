# ReMake

The GNU Make replacement for the 21st Century.

## Usage

ReMake uses a `tasks.toml` file to define tasks that can be run
with the same command syntax.

``` toml
[build]
command="cargo build"
```

and then run with

```
$ remake build
```

You can also add the `quiet` flag to the build task to supress `stdout` output.


``` toml
[build]
command="cargo build"
quiet=true
```

When performing more compilicated build processes, you may require `dependency` tasks. A dependency task will execute, prior to the task called for example...

``` toml
[build]
command="cargo build"
dependencies=["clean"]

[clean]
command="cargo clean"
quiet=true
```

This will run the `clean` task before the `build`.

## Installation

__Coming Soon__

## Todo

- [x] Dependant tasks
- [ ] Targets (replicate `.PHONY` functionality from traditional `Makefiles`)
