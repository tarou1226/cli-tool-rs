use std::fs::File;
use std::io::{Result, Error, ErrorKind};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct AddOptions {
    #[structopt(long)]
    file: String,
}

pub fn add(folder: &str, args: &AddOptions) -> Result<()> {
    let path = folder.to_string() + &args.file;
    // error handling for AlreadyExists.
    if Path::new(&path).exists() {
        Err(Error::new(ErrorKind::AlreadyExists, "already existed this file."))
    } else {
        File::create(path).expect("failed to create file");
        Ok(())
    }
}
