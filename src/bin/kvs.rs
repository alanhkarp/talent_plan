
use std::process::exit;

use clap::{App, Arg, SubCommand};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let matches = App::new("Key Value Store")
        .version(version)
        .author(author)
        .about("Pingcap Talent Plan")
        .subcommand(
            SubCommand::with_name("get")
                .help("get key - Get value associated with string key")
                .arg(Arg::with_name("key").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .help("set key value - Set string value to associate with string key")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .help("rm key - Get value associated with string key")
                .arg(Arg::with_name("key").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            eprintln!("Must specify a command");
            exit(1);
        }
    }
}
