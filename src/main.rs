extern crate structopt;

use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Depth (0 = max)
    #[structopt(short = "d", long = "depth", default_value = "1")]
    depth: u8
}

fn main() {
    // TODO: handle error
    let path = get_path().unwrap();

    let opt = Opt::from_args();

    if !&path.is_dir() {
        println!("Invalid path");
        std::process::exit(1);
    }

    let depth = if opt.depth == 0 {None} else {Some(opt.depth)};

    walk_folder(path, 0, depth)
}

fn walk_folder(folder_path: PathBuf, current_depth: usize, depth: Option<u8>) -> () {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for e in entries {
            let path = e.unwrap().path();
            if path.is_dir() {
                let next_depth = current_depth + 1;
                
                let remaining_depth = if depth.is_some() {Some(depth.unwrap() - 1)} else {None};
                if remaining_depth == Some(0) {
                    return;
                }

                println!("{}{}", "  ".repeat(current_depth), &path.display());
                walk_folder(path, next_depth, remaining_depth);

            } else {
                println!("{}{}", "  ".repeat(current_depth), &path.display());
            }
        }
    }
}



#[derive(Debug)]
pub enum Error {
//   Os(),
}

fn get_path() -> Result<PathBuf, Error> {
    if env::args_os().len() > 1 {
        let arg = env::args_os().nth(1).unwrap().into_string().unwrap();
        // TODO: properly detect if it's a path
        if !arg.starts_with('-'){
            let p =  PathBuf::from(&arg);
            return Ok(p);
        }
    }

    let current_path = env::current_dir().unwrap();
    
    Ok(current_path)
}
