use structopt::StructOpt;

#[derive(StructOpt)]
pub struct AddOptions {
    #[structopt(long)]
    file: String
}