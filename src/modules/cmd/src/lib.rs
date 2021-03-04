use clap::{App, Arg};

pub struct RxSshArgs {
    pub host: String,
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
                Arg::with_name("host")
                    .required(true)
                    .short("h")
                    .long("host")
                    .takes_value(true)
                    .help("Host to be executed on"),
            )
            .get_matches();

        let host = String::from(matches.value_of("host").unwrap());
        let command = String::from(matches.value_of("command").unwrap());

        return Ok(RxSshArgs { host, command });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
