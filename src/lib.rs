use std::net::{SocketAddrV4, UdpSocket};
use std::time::Duration;
use std::{io, thread};

const MULTICAST_ADDR: &str = "224.0.2.60:4445";

pub fn run(
    bind_addr: SocketAddrV4,
    motd: &str,
    server_port: &str,
    sleep: Duration,
) -> io::Result<()> {
    let socket = UdpSocket::bind(bind_addr)?;
    socket.connect(MULTICAST_ADDR)?;
    let buf = construct_message(motd, server_port);
    loop {
        socket.send(&buf)?;
        thread::sleep(sleep);
    }
}

fn construct_message(motd: &str, server_port: &str) -> Vec<u8> {
    format!("[MOTD]{}[/MOTD][AD]{}[/AD]", motd, server_port).into_bytes()
}
