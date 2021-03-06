use clap::{App, Arg};

pub struct RxSshArgs {
    pub hostname: String,
    pub username: String,
    pub command: String,
}

impl RxSshArgs {
    pub fn new() -> Result<RxSshArgs, &'static str> {
        let version = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");

        let matches = App::new("RxSSH")
            .version(version)
            .author(authors)
            .about("https://rxssh.com")
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

        let command = String::from(
            matches
                .value_of("command")
                .expect("Error while handling 'command' argument."),
        );
        let username = String::from(
            matches
                .value_of("username")
                .expect("Error while handling 'username' argument."),
        );
        let hostname = String::from(
            matches
                .value_of("hostname")
                .expect("Error while handling 'hostname' argument."),
        );

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
