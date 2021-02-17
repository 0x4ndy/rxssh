use std::env;
use std::process;

use rxssh::RxSshArgs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rx_ssh_args = RxSshArgs::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rxssh::run(rx_ssh_args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
