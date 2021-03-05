use std::error::Error;
use std::{io::Read, net::TcpStream};
// TODO uncomment this 'use' once the execute_block() is implemented
// use std::{
//     io::{Read, Write},
// };

use ssh2::Session;

pub fn execute_single(
    hostname: String,
    username: String,
    command: String,
) -> Result<String, Box<dyn Error>> {
    let tcp = TcpStream::connect(format!("{}:22", &hostname)).unwrap();
    let mut session = Session::new().unwrap();

    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_agent(&username).unwrap();

    let mut channel = session.channel_session().unwrap();
    channel.exec(&command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();

    let code = channel.exit_status().unwrap();

    if code != 0 {
        eprintln!("Execution error: {}", code);
    }

    Ok(output)
}

// This function executes a block of ssh commands
// TODO: refactor this function

// pub fn execute_block() -> Result<(), Box<dyn Error>> {
//     let tcp = TcpStream::connect(format!("{}:22", rx_ssh_args.host)).unwrap();
//     let mut sess = Session::new().unwrap();

//     sess.set_tcp_stream(tcp);
//     sess.handshake().unwrap();
//     sess.userauth_agent(&rx_ssh_args.username).unwrap();

//     let mut channel = sess.channel_session().unwrap();
//     channel.shell().unwrap();

//     channel
//         .write(format!("{}\n", rx_ssh_args.command).as_bytes())
//         .unwrap();
//     let mut buf = [1u8; 16000];
//     channel.read(&mut buf).unwrap();
//     let s = String::from_utf8_lossy(&buf);
//     println!("{}", s);

//     channel.wait_close();
//     println!("{}", channel.exit_status().unwrap());

//     Ok(())
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
