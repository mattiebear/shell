# Rust Shell

A minimal Unix shell implementation written in Rust, built as a learning project to understand shell internals and Rust systems programming.

## Features

- **Built-in commands:**
  - `echo` - Display text
  - `exit` - Exit the shell
  - `type` - Display command type information (planned)

- Clean command parsing and execution architecture
- Interactive REPL with command prompt

## How It Works

The shell uses an enum-based command representation where each built-in command is a variant of the `Command` enum. Commands are parsed from user input, matched against known builtins, and executed with a simple pattern matching system.

The architecture separates parsing (`Command::parse()`) from execution, allowing for clean separation of concerns and easy addition of new commands.

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Project Status

Active development - currently implementing core shell built-ins and exploring architectural patterns for command execution and control flow.
