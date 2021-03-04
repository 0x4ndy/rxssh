use std::{error::Error, io::{Read, Write}};
use std::net::TcpStream;

use ssh2::Session;

use cmd::RxSshArgs;

pub fn run(rx_ssh_args: RxSshArgs) -> Result<(), Box<dyn Error>> {
    println!("Host: {}", rx_ssh_args.host);
    println!("Command: {}", rx_ssh_args.command);

    let tcp = TcpStream::connect("andy.codes:22").unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent("andy").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.shell().unwrap();

    channel.write("cd tmp; pwd\n".as_bytes()).unwrap();
    let mut buf = [1u8; 16000];
    channel.read(&mut buf).unwrap();
    let s = String::from_utf8_lossy(&buf);
    println!("{}", s);

    channel.write("ls\n".as_bytes()).unwrap();
    // let mut buf = [1u8; 16000];
    channel.read(&mut buf).unwrap();
    let s = String::from_utf8_lossy(&buf);
    println!("{}", s);

    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());


    Ok(())
}
