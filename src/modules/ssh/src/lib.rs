use std::{error::Error, io::Read, net::TcpStream};

use anyhow::Result;
use ssh2::Session;

// Static values
static SSH_PORT: u8 = 22;

pub fn execute_single(
    hostname: &str,
    username: &str,
    command: &str,
) -> Result<String, Box<dyn Error>> {
    let tcp = TcpStream::connect(format!("{}:{}", hostname, SSH_PORT))?;
    let mut session = Session::new()?;

    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_agent(username)?;

    let mut channel = session.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;

    match channel.exit_status() {
        Ok(code) => exit_code_to_string(code, &mut output),
        Err(err) => return Err(err.into()),
    };

    Ok(output)
}

fn exit_code_to_string(code: i32, output: &mut String) {
    if code != 0 {
        output.clear();
        // TODO: Handle this by a switch on different code
        //       values and return a string interpretation
        output.push_str(&code.to_string());
    }
}
