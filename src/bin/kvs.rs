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
                .about("get key - Get value associated with key")
                .arg(Arg::with_name("key").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set key value - Set value to associate with key")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("rm key - Get value associated with key")
                .arg(Arg::with_name("key").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_m)) => {
            eprint!("unimplemented");
            exit(1);
        }
        ("set", Some(_m)) => {
            eprint!("unimplemented");
            exit(1);
        }
        ("rm", Some(_m)) => {
            eprint!("unimplemented");
            exit(1);
        }
        _ => {
            eprintln!("Must specify a command");
            exit(1);
        }
    }
}
