use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        input.clear();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(command) = ShellCommand::parse(&input.trim().to_string()) {
            match command.execute() {
                ShellCommandResult::Continue => continue,
                ShellCommandResult::Exit => break,
            }
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}

enum ShellCommandResult {
    Continue,
    Exit,
}

#[derive(Clone)]
struct Executable {
    name: String,
    path: PathBuf,
}

enum ShellCommandType {
    Echo,
    Exec(Executable),
    Exit,
    Type,
}

struct ShellCommand {
    args: Vec<String>,
    command_type: ShellCommandType,
}

impl ShellCommand {
    fn parse(input: &String) -> Result<Self, String> {
        let arg_list: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        if let [name, rest @ ..] = arg_list.as_slice() {
            if let Some(command_type) = Self::get_builtin(name) {
                Ok(ShellCommand {
                    args: rest.to_vec(),
                    command_type,
                })
            } else if let Some(executable) = Self::find_executable(name) {
                Ok(ShellCommand {
                    args: rest.to_vec(),
                    command_type: ShellCommandType::Exec(executable),
                })
            } else {
                Err(format!("Command not found: {}", name))
            }
        } else {
            Err("No command provided".to_string())
        }
    }

    fn is_builtin(name: &str) -> bool {
        match Self::get_builtin(name) {
            Some(_) => true,
            None => false,
        }
    }

    fn get_builtin(name: &str) -> Option<ShellCommandType> {
        match name {
            "echo" => Some(ShellCommandType::Echo),
            "type" => Some(ShellCommandType::Type),
            "exit" => Some(ShellCommandType::Exit),
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

    fn execute(&self) -> ShellCommandResult {
        match &self.command_type {
            ShellCommandType::Echo => {
                println!("{}", self.args.join(" "));
                ShellCommandResult::Continue
            }
            ShellCommandType::Exec(executable) => {
                let status = Command::new(executable.path.to_str().unwrap())
                    .arg0(&executable.name)
                    .args(&self.args)
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status();

                match status {
                    Ok(_) => ShellCommandResult::Continue,
                    Err(e) => {
                        eprintln!("Failed to execute {}: {}", executable.name, e);
                        ShellCommandResult::Continue
                    }
                }
            }
            ShellCommandType::Exit => ShellCommandResult::Exit,
            ShellCommandType::Type => {
                if ShellCommand::is_builtin(&self.args.first().unwrap()) {
                    println!("{} is a shell builtin", &self.args.first().unwrap());
                } else if let Some(exec) =
                    ShellCommand::find_executable(&self.args.first().unwrap())
                {
                    println!("{} is {}", exec.name, exec.path.display());
                } else {
                    println!("{}: not found", &self.args.first().unwrap());
                }

                ShellCommandResult::Continue
            }
        }
    }
}
