use clap::{App, Arg};
use futures::{stream, StreamExt};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::net::TcpStream;

mod ports;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli_matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("1002"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Display detailed information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("full")
                .help("Scan all 65535 ports")
                .long("full"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t")
                .default_value("3"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let full = cli_matches.is_present("full");
    let verbose = cli_matches.is_present("verbose");
    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1002);
    let timeout = cli_matches
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(3);
    let target = cli_matches.value_of("target").unwrap();

    if verbose {
        let ports = if full {
            String::from("all the 65535 ports")
        } else {
            String::from("the most common 1002 ports")
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}",
            &ports, target, concurrency, timeout
        );
    }

    scan(target, full, concurrency, timeout).await;

    Ok(())
}

async fn scan(target: &str, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(hostname: &str, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return;
    }

    if tokio::time::timeout(timeout, TcpStream::connect(&socket_addresses[0]))
        .await
        .is_ok()
    {
        println!("{}", port);
    }
}

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}
