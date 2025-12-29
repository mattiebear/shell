use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        input.clear();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(command) = Command::parse(input.trim().to_string()) {
            match command.execute() {
                CommandResult::Continue => continue,
                CommandResult::Exit => break,
            }
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}

enum CommandResult {
    Continue,
    Exit,
}

enum Command {
    Echo(String),
    Exit,
    Type(String),
}

impl Command {
    fn parse(input: String) -> Option<Self> {
        if let Some(split_index) = input.find(" ") {
            let (name, args) = input.split_at(split_index);

            match name {
                "echo" => Some(Command::Echo(args.trim().to_string())),
                "type" => Some(Command::Type(args.trim().to_string())),
                _ => None,
            }
        } else {
            match input.as_str() {
                "exit" => Some(Command::Exit),
                _ => None,
            }
        }
    }

    fn is_builtin(name: &str) -> bool {
        matches!(name, "echo" | "type" | "exit")
    }

    fn execute(&self) -> CommandResult {
        match self {
            Command::Echo(args) => {
                println!("{}", args);
                CommandResult::Continue
            }
            Command::Exit => CommandResult::Exit,
            Command::Type(args) => {
                if Command::is_builtin(args) {
                    println!("{} is a shell builtin", args);
                } else {
                    println!("{}: not found", args);
                }

                CommandResult::Continue
            }
        }
    }
}
