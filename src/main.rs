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

        if let Ok(command) = Command::parse(&input.trim().to_string()) {
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

#[derive(Clone)]
struct Executable {
    name: String,
    path: PathBuf,
}

enum CommandType {
    Echo,
    Exec(Executable),
    Exit,
    Type,
}

struct Command {
    args: Vec<String>,
    command_type: CommandType,
}

impl Command {
    fn parse(input: &String) -> Result<Self, String> {
        let arg_list: Vec<String> = input
            .split(" ")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let rest = arg_list[1..].to_vec();

        if let Some(name) = arg_list.first() {
            if Self::is_builtin(name) {
                let command_type = Self::get_builtin(name).unwrap();
                Ok(Command {
                    args: rest,
                    command_type,
                })
            } else {
                if let Some(executable) = Self::find_executable(name) {
                    return Ok(Command {
                        args: rest,
                        command_type: CommandType::Exec(executable),
                    });
                } else {
                    Err(format!("Command not found: {}", name))
                }
            }
        } else {
            Err("No command provided".to_string())
        }
    }

    fn is_builtin(name: &str) -> bool {
        matches!(name, "echo" | "type" | "exit")
    }

    fn get_builtin(name: &str) -> Option<CommandType> {
        match name {
            "echo" => Some(CommandType::Echo),
            "type" => Some(CommandType::Type),
            "exit" => Some(CommandType::Exit),
            _ => None,
        }
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
        match &self.command_type {
            CommandType::Echo => {
                println!("{}", self.args.join(" "));
                CommandResult::Continue
            }
            CommandType::Exec(executable) => {
                println!("Executing {}", executable.name);
                CommandResult::Continue
            }
            CommandType::Exit => CommandResult::Exit,
            CommandType::Type => {
                if Command::is_builtin(&self.args.first().unwrap()) {
                    println!("{} is a shell builtin", &self.args.first().unwrap());
                } else if let Some(exec) = Command::find_executable(&self.args.first().unwrap()) {
                    println!("{} is {}", exec.name, exec.path.display());
                } else {
                    println!("{}: not found", &self.args.first().unwrap());
                }

                CommandResult::Continue
            }
        }
    }
}
