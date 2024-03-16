use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod plugin;

const YELLOW: &str = "\x1b[0;33m";
const RESET: &str = "\x1b[0m";
const VERSION: &str = "hsh (Hubschluft Shell) 0.2v";

mod config {
    use std::process::{Command, Stdio};
    pub fn shortcuts(command: &str, args: &[&str]) -> Result<bool, std::io::Error> {
        if command == "pacin" {
            let mut cmd = Command::new("sudo");
            cmd.arg("pacman").arg("-S").args(args);

            match cmd.stdin(Stdio::inherit())
                     .stdout(Stdio::inherit())
                     .stderr(Stdio::inherit())
                     .spawn() {
                Ok(mut child) => {
                    child.wait()?;
                    Ok(true) 
                }
                Err(e) => Err(e), 
            }
        } else {
            Ok(false)
        }
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("OK");

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

        if command == "exit" {
            break;
        } else if command == "cd" {
            let new_dir = args.first().map_or("/", |&x| x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
            continue;
        } else if command == "help" {
            let help: &str = r#"
help    print the help menu
version output version information
            "#;
            println!("{}", help)
        } else if command == "version" {
            println!("{}", VERSION)
        }

        match config::shortcuts(command, &args) {
            Ok(false) => {
                match Command::new(command)
                    .args(&args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn() {
                    Ok(mut child) => {
                        child.wait().expect("Failed to wait for child process");
                    }
                    Err(e) => {
                        println!("Failed to execute command: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to execute command: {}", e);
            }
            _ => {}
        }

        while running.load(Ordering::SeqCst) {
            continue;
        }
    }
}
