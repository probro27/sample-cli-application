# Command Line Application

A clone for the popular `grep` command that takes in a pattern and a file path to give all of the lines with the occurances.

Right now only default patterns are supported with exact match.

## Installation

You can install the project using the `cargo install` command.

```bash
cargo install <package_name>
```

Note: Here replace `<package_name>` with whatever the name of the package is :)

## Running locally

To run, please use:

```bash
cargo run -- <pattern> <file_path>
```

```bash
cargo run -- main main.rs
```

## Testing

There are some sample tests in the files `unit.rs` and `cli.rs` which check for unit tests and CLI test respectively.
Feel free to run them if you decide to contribute!
