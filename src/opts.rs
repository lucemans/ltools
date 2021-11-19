use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ltools", setting = structopt::clap::AppSettings::ArgRequiredElseHelp)]
pub enum Opts {
    /// Lists all directories in da thing
    Dev {
        /// Directory to search for projects in
        #[structopt(short = "d", long = "directory")]
        dir: Option<String>,

        // Show hidden files
        #[structopt(short = "a", long = "all")]
        all: bool
    },
}
