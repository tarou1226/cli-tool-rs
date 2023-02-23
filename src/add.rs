use std::fs::File;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct AddOptions {
    #[structopt(long)]
    file: String,
}

pub fn add(args: &AddOptions) -> Result<()> {
    let folder = "./files/";
    let path = folder.to_string() + &args.file;
    File::create(path).expect("failed to create file");
    Ok(())
}
