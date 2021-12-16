use clap::{Parser, Subcommand};
use std::ops::RangeInclusive;

// Constants
const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

// Helpers
fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start server
    Serve {
        /// Select tcp port
        #[arg(short, long, value_parser = port_in_range, default_value_t = 2023)]
        port: u16,

        /// Select tcp host
        #[arg(long, default_value = "localhost")]
        host: String,

        /// Peers to connect
        #[arg(long, value_name = "PEERS")]
        peers: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Serve { port, host, peers }) => {
            println!("Start server on port {:?}...", port);
        }
        None => {}
    }

    // Continued program logic goes here...
}
