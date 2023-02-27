use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct RemoveOptions {
    #[structopt(long)]
    file: String,
}

pub fn remove(folder: &str, args: &RemoveOptions) -> Result<()> {
    let path = folder.to_string() + &args.file;
    // error handling for NotFound.
    if Path::new(&path).exists() {
        let file_path = Path::new(&path);
        remove_file(file_path).unwrap();
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "Not found this file."))
    }
}
