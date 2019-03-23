extern crate structopt;

use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Depth (0 = max)
    #[structopt(short = "d", long = "depth", default_value = "1")]
    depth: u8,

    /// Root directory
    directory: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let path = get_path(opt.directory);

    match path {
        Ok(p) => {
            let depth = if opt.depth == 0 {None} else {Some(opt.depth)};
            walk_folder(p, 1, depth);
        },
        Err(e) => handle_error(e),
    };
}

fn handle_error(err: Error) {
    println!("{}", err.details);
    std::process::exit(1);
}

fn walk_folder(folder_path: PathBuf, current_depth: usize, depth: Option<u8>) -> () {
    if let Ok(entries) = fs::read_dir(&folder_path) {

        if current_depth == 1 {
            println!("{}", &folder_path.display());
        }

        for e in entries {
            let path = e.unwrap().path();
            if path.is_dir() {
                let next_depth = current_depth + 1;
                
                let remaining_depth = if depth.is_some() {Some(depth.unwrap() - 1)} else {None};
                if remaining_depth == Some(0) {
                    return;
                }

                println!("{}{}", "  ".repeat(current_depth), path.file_name().unwrap().to_string_lossy());

                walk_folder(path, next_depth, remaining_depth);

            } else {
                println!("{}{}", "  ".repeat(current_depth), &path.file_name().unwrap().to_string_lossy());
            }
        }
    }
}

fn get_path(requested_path: Option<String>) -> Result<PathBuf, Error> {
    match requested_path {
        Some(p) => {
            let path = PathBuf::from(p);

            if path.is_dir() {
                return Ok(path)
            }

            let message = format!("{} is not a directory", path.display());

            return Err(Error::new(&message))

        },
        None => {
            return Ok(env::current_dir().unwrap())
        },
    };
}

#[derive(Debug)]
struct Error {
    details: String
}

impl Error {
    fn new(msg: &str) -> Error {
        Error{details: msg.to_string()}
    }
}
