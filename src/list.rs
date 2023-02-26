use std::fs::read_dir;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct ListOptions {
    #[structopt(long)]
    sorted: bool,
}

pub fn list(folder: &str, args: &ListOptions) -> Result<()> {
    let mut paths: Vec<_> = read_dir(&folder)
        .expect("failed to read directory")
        .filter_map(Result::ok)
        .collect();

    if args.sorted {
        paths.sort_by_key(|dir| dir.file_name())
    }

    for path in paths {
        println!("{}", path.file_name().to_str().unwrap())
    }

    Ok(())
}
