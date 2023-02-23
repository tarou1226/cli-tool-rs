use cli_tools::add::add;
use cli_tools::cli::{Cli, Command};
use structopt::StructOpt;

fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    match args.cmd {
        Command::Add(args) => add(&args),
    }
}
