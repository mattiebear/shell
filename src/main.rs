use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        input.clear();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(command) = parse_command(input.trim().to_string()) {
            match command {
                Command::Echo(args) => println!("{}", args),
                Command::Exit => break,
            }
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}

enum Command {
    Echo(String),
    Exit,
}

fn parse_command(input: String) -> Option<Command> {
    if let Some(split_index) = input.find(" ") {
        let (name, args) = input.split_at(split_index);

        match name {
            "echo" => Some(Command::Echo(args.to_string())),
            _ => None,
        }
    } else {
        match input.as_str() {
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }
}
