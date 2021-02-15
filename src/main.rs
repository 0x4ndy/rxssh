use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let output = Command::new("ls").args(&["."]).status().expect("Unable to execute this process.");

    println!("{}", output.to_string());
}
