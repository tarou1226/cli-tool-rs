use crate::add::AddOptions;
use crate::list::ListOptions;
use crate::remove::RemoveOptions;
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
    #[structopt(about = "show all files", name = "list")]
    List(ListOptions),
    #[structopt(about = "remove a file", name = "remove")]
    Remove(RemoveOptions),
    /*
    Rename(),
    */
}
