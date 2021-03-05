use std::error::Error;

use cmd::RxSshArgs;

pub fn run(rx_ssh_args: RxSshArgs) -> Result<(), Box<dyn Error>> {
    let results = ssh::execute_single(rx_ssh_args.host, rx_ssh_args.username, rx_ssh_args.command);

    println!("{}", results.unwrap().to_string());

    Ok(())
}
