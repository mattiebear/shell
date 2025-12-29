use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

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

struct Executable {
    name: String,
    path: PathBuf,
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

    fn find_executable(name: &str) -> Option<Executable> {
        let paths: Vec<PathBuf> = if let Some(path_var) = env::var_os("PATH") {
            env::split_paths(&path_var).collect()
        } else {
            Vec::new()
        };

        for path in paths.iter() {
            let file = path.join(name);

            if file.is_file() {
                if let Ok(metadata) = file.metadata() {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        return Some(Executable {
                            name: name.to_string(),
                            path: file,
                        });
                    }
                }
            }
        }

        None
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
                } else if let Some(exec) = Command::find_executable(args) {
                    println!("{} is {}", exec.name, exec.path.display());
                } else {
                    println!("{}: not found", args);
                }

                CommandResult::Continue
            }
        }
    }
}
