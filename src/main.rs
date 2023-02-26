use cli_tools::add::add;
use cli_tools::cli::{Cli, Command};
use cli_tools::list::list;
use structopt::StructOpt;

fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    let folder = "./files/";
    match args.cmd {
        Command::Add(args) => add(folder, &args),
        Command::List(args) => list(folder, &args),
    }
}
