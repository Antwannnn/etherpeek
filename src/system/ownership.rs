use std::process::{Command, ExitStatus};

pub fn transfer_file_ownership(path: &String) -> std::io::Result<ExitStatus> {
    let username = std::env::var("SUDO_USER")
        .unwrap_or_else(|_| whoami::username()); // add `whoami = "1.2"` to Cargo.toml
    
    Command::new("chown")
        .arg(format!("{}", username)) // or a specific group
        .arg(path)
        .status()
}