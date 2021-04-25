use std::net::SocketAddrV4;
use std::process;

use clap::Clap;
use std::time::Duration;

/// Fakes discovery messages of LAN Minecraft servers.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Niklas Elsbrock <mail@nelsbrock.de>")]
struct Opts {
    /// "Message of the day" to use. This supports Minecraft formatting codes.
    motd: String,

    /// The port to advertise.
    ///
    /// This does not need to be a valid port or number. It supports Minecraft formatting codes.
    #[clap(short = 'p', long, default_value = "0")]
    server_port: String,

    /// The bind address to use for the UDP socket.
    #[clap(long, default_value = "0.0.0.0:34524")]
    udp_bind: SocketAddrV4,

    /// The amount of seconds to wait after each send operation.
    #[clap(long, default_value = "1.5")]
    interval: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    match mclanfake::run(
        opts.udp_bind,
        &opts.motd,
        &opts.server_port,
        Duration::from_secs_f64(opts.interval),
    ) {
        Ok(_) => unreachable!(),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
}
