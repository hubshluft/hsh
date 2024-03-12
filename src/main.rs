const YELLOW: &str = "\x1b[0;33m";
const RESET: &str = "\x1b[0m";

use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

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

        match Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut child) => {
                child.wait().expect("Failed to wait for child process");
            }
            Err(e) => {
                println!("Failed to execute command: {}", e);
            }
        }
    }
}
