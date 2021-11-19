// THIS PACKAGE IS WORK IN PROGRESS, PLEASE DO NOT DELETE FROM AUR
use structopt::StructOpt;
mod opts;

fn main() {
    let _args = opts::Opts::from_args();

    match opts::Opts::from_args() {
        opts::Opts::Dev { all, dir } => {
            println!("hello");
            if all {
                println!("Showing all files");
            }
            if let Some(dir) = dir {
                println!("Searching in {}", dir);
            }
        }
    }
}
