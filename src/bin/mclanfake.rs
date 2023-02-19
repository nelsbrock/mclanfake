use clap::builder::{NonEmptyStringValueParser, TypedValueParser};
use std::ffi::OsString;
use std::net::SocketAddrV4;
use std::process::ExitCode;
use std::string::ToString;
use std::time::Duration;

use clap::Parser;
use mclanfake::Repeat;
use os_str_bytes::OsStrBytes;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Advertise this "message of the day" to clients.
    ///
    /// This supports Minecraft formatting codes.
    #[arg(
        short,
        long,
        value_parser,
        value_name = "STRING",
        default_value = "A Minecraft Server"
    )]
    motd: OsString,

    /// Advertise this "port" to clients.
    ///
    /// This does not need to be a valid port or number,
    /// and Minecraft formatting codes are supported.
    #[arg(
        short,
        long,
        value_parser,
        value_name = "STRING",
        default_value = "25565"
    )]
    port: OsString,

    /// Only send one advertisement message, then exit.
    #[arg(long, conflicts_with = "interval")]
    once: bool,

    /// Wait this number of seconds after sending each advertisement message.
    #[arg(
        long,
        value_name = "SECONDS",
        value_parser = NonEmptyStringValueParser::new()
            .try_map(
                |s| s.parse::<f64>().map_err(|e| e.to_string())
                    .and_then(|f| Duration::try_from_secs_f64(f).map_err(|e| e.to_string()))
            ),
        default_value = "1.5"
    )]
    interval: Duration,

    /// Bind the UDP socket to this address.
    #[arg(long, value_name = "ADDRESS", default_value = "0.0.0.0:0")]
    bind: SocketAddrV4,
}

fn main() -> ExitCode {
    let args: Args = Args::parse();

    let repeat = if args.once {
        Repeat::Once
    } else {
        Repeat::Forever {
            interval: args.interval,
        }
    };

    match mclanfake::run(
        args.bind,
        &args.motd.to_raw_bytes(),
        &args.port.to_raw_bytes(),
        repeat,
    ) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
