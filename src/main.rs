const YELLOW: &str = "\x1b[0;33m";
const RESET: &str = "\x1b[0m";

// TODO: the ability to execute TUI software

use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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

        let path_str = path.to_string_lossy();
        let old_v = path_str.replace("/home/", "~/");

        print!(
            "{YELLOW}{}{RESET}@{YELLOW}{}{RESET} $ ",
            user,
            old_v,
            YELLOW = YELLOW,
            RESET = RESET
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
        let output = Command::new(command_path).args(args).output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_stdout = String::from_utf8_lossy(&output.stdout);
                    println!("{}", output_stdout);
                } else {
                    let output_stderr = String::from_utf8_lossy(&output.stderr);
                    println!("{}", output_stderr);
                }
            }
            Err(e) => {
                println!("Failed to execute command: {}", e);
            }
        }
    }
}
