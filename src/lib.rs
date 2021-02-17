use std::error::Error;
// use std::process::Command;

pub struct RxSshArgs {
    pub host: String,
    pub command: String,
}

impl RxSshArgs {
    pub fn new(args: &[String]) -> Result<RxSshArgs, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let host = args[1].clone();
        let command = args[2].clone();

        Ok(RxSshArgs { host, command })
    }
}

pub fn run(rx_ssh_args: RxSshArgs) -> Result<(), Box<dyn Error>> {
    println!("Host: {}", rx_ssh_args.host);
    println!("Command: {}", rx_ssh_args.command);

    // TODO: implement the execution of the command. The code below is just an example

    // let output = Command::new("ls").args(&["."]).status().expect("Unable to execute this process.");
    // println!("{}", output.to_string());

    Ok(())
}
