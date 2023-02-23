use structopt::StructOpt;
use crate::add::AddOptions;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about="add a file", name="add")]
    Add(AddOptions),
    /*List(),
    Remove(),
    Rename(),*/
}