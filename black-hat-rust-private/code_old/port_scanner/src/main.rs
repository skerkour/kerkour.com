use clap::{App, Arg};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("Target's IP address")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .long("concurrency")
                .short("c")
                .help("Number of threads")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("from")
                .long("from")
                .help("Starting port")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("to")
                .long("to")
                .help("Ending port")
                .default_value("65535"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    // we check that concurrency is an u16
    let concurrency = matches.value_of("concurrency").unwrap().parse::<u16>()?;
    // and we configure rayon's threadpool accordingly
    env::set_var("RAYON_NUM_THREADS", concurrency.to_string());

    // we can safely unwrap because argument is marked as required
    let target = matches.value_of("target").unwrap();
    println!("target: {}", target);

    let starting_port = matches.value_of("from").unwrap().parse::<u16>()?;
    let ending_port = matches.value_of("to").unwrap().parse::<u16>()?;

    if starting_port > ending_port {
        return Err("Starting port can't be greater than ending port".into());
    }

    let open_ports: Vec<(u16, bool)> = (starting_port..=ending_port)
        .into_par_iter()
        .map(|port| {
            println!("scanning: {}", port);
            (port, true)
        })
        .filter(|port| port.1)
        .collect();

    for open_port in open_ports {
        println!("{}: open", open_port.0)
    }

    Ok(())
}
