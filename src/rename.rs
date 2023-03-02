use std::fs::rename;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct RenameOptions {
    before: String,
    after: String,
}

pub fn rename_file(folder: &str, args: &RenameOptions) -> Result<()> {
    let before_path = folder.to_string() + &args.before;
    let after_path = folder.to_string() + &args.after;
    // error handling for NotFound and AlreadyExists.
    if Path::new(&before_path).exists() && !Path::new(&after_path).exists() {
        rename(before_path, after_path).unwrap();
        Ok(())
    } else if Path::new(&after_path).exists() {
        Err(Error::new(ErrorKind::AlreadyExists, "Already exist the after file name"))
    } else {
        Err(Error::new(ErrorKind::NotFound, "Not found this file."))
    }
}
