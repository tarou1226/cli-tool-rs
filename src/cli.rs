use crate::add::AddOptions;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "add a file", name = "add")]
    Add(AddOptions), 
    /*
    List(),
    Remove(),
    Rename(),
    */
}
