use std::process;
use cmd::RxSshArgs;

fn main() {
    let rx_ssh_args = RxSshArgs::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rxssh::run(rx_ssh_args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
