// THIS PACKAGE IS WORK IN PROGRESS, PLEASE DO NOT DELETE FROM AUR
use structopt::StructOpt;
mod opts;

fn main() {
    let args = opts::Opts::from_args();
    // if let Some(command) = args.command {
    //     println!("{}", command);        
    // } else {
    //     println!("No command given");
    // }
}
