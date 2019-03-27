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

    /// Include hidden directories that begin with a dot (.)
    #[structopt(short = "a", long = "all")]
    all: bool,

    /// Root directory
    directory: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let path = get_path(opt.directory);

    match path {
        Ok(p) => {
            let depth = if opt.depth == 0 {None} else {Some(opt.depth)};
            walk_folder(p, 1, depth, opt.all);
        },
        Err(e) => handle_error(e),
    };
}

fn handle_error(err: Error) {
    println!("{}", err.details);
    std::process::exit(1);
}

fn walk_folder(folder_path: PathBuf, current_depth: usize, depth: Option<u8>, show_hidden: bool) -> () {
    if let Ok(entries) = fs::read_dir(&folder_path) {

        if current_depth == 1 {
            println!("{}", &folder_path.display());
        }

        for e in entries {
            let path = e.unwrap().path();

            let file_name = path.file_name().unwrap().to_string_lossy().to_string();

            if !show_hidden && is_hidden(&file_name) {
                continue;
            }

            if path.is_dir() {
                let next_depth = current_depth + 1;
                
                let remaining_depth = if depth.is_some() {Some(depth.unwrap() - 1)} else {None};
                if remaining_depth == Some(0) {
                    continue;
                }

                println!("{}{}", "  ".repeat(current_depth), file_name);

                walk_folder(path, next_depth, remaining_depth, show_hidden);

            } else {
                println!("{}{}", "  ".repeat(current_depth), file_name);
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

fn is_hidden(file_or_directory: &str) -> bool {
    file_or_directory.starts_with('.')
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
