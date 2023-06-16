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

## Installation

__Coming Soon__

## Todo

- [ ] Dependant tasks
- [ ] Targets (replicate `.PHONY` functionality from traditional `Makefiles`)
