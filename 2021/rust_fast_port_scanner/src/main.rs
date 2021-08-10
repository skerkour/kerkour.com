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
    let target = cli_matches.value_of("target").unwrap();

    if verbose {
        let ports = if full {
            String::from("all 65535 ports")
        } else {
            String::from("the most 1002 common ports")
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}",
            &ports, target, concurrency
        );
    }

    scan(target, full, concurrency).await;

    Ok(())
}

async fn scan(target: &str, full: bool, concurrency: usize) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| async move {
            if scan_port(target, port).await {
                println!("{:?}", port);
            }
        })
        .await;
}

async fn scan_port(hostname: &str, port: u16) -> bool {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return false;
    }

    tokio::time::timeout(timeout, TcpStream::connect(&socket_addresses[0]))
        .await
        .is_ok()
}

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((0..u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}
