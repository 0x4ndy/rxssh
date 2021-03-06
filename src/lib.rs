use std::error::Error;

use cmd::RxSshArgs;

pub fn run(rx_ssh_args: RxSshArgs) -> Result<(), Box<dyn Error>> {
    let results = ssh::execute_single(
        &rx_ssh_args.hostname,
        &rx_ssh_args.username,
        &rx_ssh_args.command,
    );

    match results {
        Ok(content) => println!("{}", content),
        Err(err) => return Err(err.into()),
    };

    Ok(())
}
