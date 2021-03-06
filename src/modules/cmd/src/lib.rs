use clap::{App, Arg};

pub struct RxSshArgs {
    pub hostname: String,
    pub username: String,
    pub command: String,
}

impl RxSshArgs {
    pub fn new() -> Result<RxSshArgs, &'static str> {
        let matches = App::new("RxSSH")
            .version("")
            .author("")
            .about("")
            .arg(
                Arg::with_name("command")
                    .required(true)
                    .short("c")
                    .long("command")
                    .takes_value(true)
                    .help("Command to be executed"),
            )
            .arg(
                Arg::with_name("username")
                    .required(true)
                    .short("u")
                    .long("username")
                    .takes_value(true)
                    .help("User to be executed on"),
            )
            .arg(
                Arg::with_name("hostname")
                    .required(true)
                    .short("h")
                    .long("hostname")
                    .takes_value(true)
                    .help("Host to be executed on"),
            )
            .get_matches();

        let command = String::from(matches.value_of("command").unwrap());
        let username = String::from(matches.value_of("username").unwrap());
        let hostname = String::from(matches.value_of("hostname").unwrap());

        return Ok(RxSshArgs {
            hostname,
            username,
            command,
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
