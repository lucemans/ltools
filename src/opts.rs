use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ltools", setting = structopt::clap::AppSettings::ArgRequiredElseHelp)]
pub enum Opts {
    /// Lists all directories in da thing
    Dev {
        /// Text to print
        #[structopt(short, long)]
        command: Option<String>
    },
}
