use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command; 

const YELLOW: &str = "\x1b[0;33m";
const RESET: &str = "\x1b[0m";

fn main() {
    loop {
        let user = match env::var("USER") {
            Ok(val) => val,
            Err(_) => "none".to_string(),
        };

        let path = match env::current_dir() {
            Ok(val) => val,
            Err(_) => panic!("Failed to get current directory"),
        };

        print!(
            "{YELLOW}{}{RESET}@{YELLOW}{}{RESET} $ ",
            user,
            path.display()
        );
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().expect("No command entered");
        let args: Vec<&str> = parts.collect();

        let command_path = PathBuf::from(command);
        let output = Command::new(command_path)
            .args(args)
            .output();

        match output{
            Ok(output) =>{
                if output.status.success(){
                    let output_stdout = String::from_utf8_lossy(&output.stdout);
                    println!("{}", output_stdout);
                } else {
                    let output_stderr = String::from_utf8_lossy(&output.stderr);
                    println!("{}", output_stderr);
                }
            }
            Err(e) =>{
                println!("Failed to execute command: {}", e);
            }
        }
    }
}
