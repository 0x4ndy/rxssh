use std::process::Command;

fn main() {
    let output = Command::new("ls").args(&["."]).status().expect("Unable to execute this process.");

    println!("{}", output.to_string());
}
