// foo
use std::process::exit;

use structopt::StructOpt;

fn main() {
    let options = CliCmd::from_args();
    match options.cmd {
        Command::Get { key } => {
            eprintln!("unimplemented: get {}", key);
            exit(1);
        }
        Command::Set { key, value } => {
            eprintln!("unimplemented: set {} {}", key, value);
            exit(1);
        }
        Command::Rm { key } => {
            eprintln!("unimplemented: rm {}", key);
            exit(1);
        }
    }
}
#[derive(Debug, StructOpt)]
#[structopt(name = "kvs")]
struct CliCmd {
    kvs: String,
    #[structopt(subcommand)]
    cmd: Command,
}
#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get")]
    Get {
        #[structopt(required = true)]
        key: String,
    },
    #[structopt(name = "set")]
    Set {
        #[structopt(required = true)]
        key: String,
        #[structopt(required = true)]
        value: String,
    },
    #[structopt(name = "rm")]
    Rm {
        #[structopt(required = true)]
        key: String,
    },
}
