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
        Ok(code) => exit_code_to_string(code, &mut output, command),
        Err(err) => return Err(err.into()),
    };

    Ok(output)
}

fn exit_code_to_string(code: i32, output: &mut String, command: &str) {
    match code {
        0 => {
            // All good here
        }
        1 => {
            output.push_str(
                &(format!(
                    "{}: exit with code {}; catchall for general errors.",
                    command, code
                )),
            );
        }
        2 => {
            output.push_str(
                &(format!(
                    "{}: exit with code {}; misuse of shell built-ins.",
                    command, code
                )),
            );
        }
        126 => {
            output.push_str(
                &(format!(
                    "{}: exit with code {}; command invoked cannot execute.",
                    command, code
                )),
            );
        }
        127 => {
            output.push_str(&(format!("{}: exit with code {}; command not found.", command, code)));
        }
        128 => {
            output.push_str(
                &(format!(
                    "{}: exit with code {}; invalid argument to exit.",
                    command, code
                )),
            );
        }
        129..=255 => {
            let signal = code - 128;
            output.push_str(
                &(format!(
                    "{}: exit with code {}; fatal error signal {}.",
                    command, code, signal
                )),
            );
        }
        _ => {
            output.push_str(&(format!("{}: exit with code {}; Unknown error.", command, code)));
        }
    }
}
