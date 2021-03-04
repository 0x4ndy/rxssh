use std::net::TcpStream;
use std::{
    error::Error,
    io::{Read, Write},
};

use ssh2::Session;

use cmd::RxSshArgs;

pub fn run(rx_ssh_args: RxSshArgs) -> Result<(), Box<dyn Error>> {
    let tcp = TcpStream::connect(format!("{}:22", rx_ssh_args.host)).unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(&rx_ssh_args.username).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.shell().unwrap();

    channel
        .write(format!("{}\n", rx_ssh_args.command).as_bytes())
        .unwrap();
    let mut buf = [1u8; 16000];
    channel.read(&mut buf).unwrap();
    let s = String::from_utf8_lossy(&buf);
    println!("{}", s);

    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());

    Ok(())
}
