use format_bytes::format_bytes;
use std::net::{SocketAddrV4, UdpSocket};
use std::time::Duration;
use std::{io, thread};

const ANNOUNCEMENT_ADDR: &str = "224.0.2.60:4445";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Repeat {
    Once,
    Forever { interval: Duration },
}

fn construct_message(motd: &[u8], advertise_port: &[u8]) -> Vec<u8> {
    format_bytes!(b"[MOTD]{}[/MOTD][AD]{}[/AD]", motd, advertise_port)
}

pub fn run(
    bind_addr: SocketAddrV4,
    motd: &[u8],
    advertise_port: &[u8],
    repeat: Repeat,
) -> io::Result<()> {
    let socket = UdpSocket::bind(bind_addr)?;
    socket.connect(ANNOUNCEMENT_ADDR)?;
    let buf = construct_message(motd, advertise_port);

    match repeat {
        Repeat::Once => socket.send(&buf).map(|_| ()),
        Repeat::Forever { interval } => loop {
            let result = socket.send(&buf);
            if let Err(err) = result {
                eprintln!("error: {err}");
            }
            thread::sleep(interval);
        },
    }
}
