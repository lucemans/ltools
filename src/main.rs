// THIS PACKAGE IS WORK IN PROGRESS, PLEASE DO NOT DELETE FROM AUR
use colored::Colorize;

struct SubCommandArg {
    alias: &'static str,
    default: &'static str,
    desc: &'static str,
}
struct SubCommand {
    alias: &'static str,
    desc: &'static str,
    args: Vec<SubCommandArg>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: lt [OPTION]");
        println!(
            "{}",
            "A general set of tools pertaining to the lucemans development system.".dimmed()
        );
        println!("");

        let commands: Vec<SubCommand> = Vec::from([SubCommand {
            alias: "dev",
            desc: "Get a list of development directories",
            args: Vec::from([
                SubCommandArg {
                    alias: "dir",
                    default: "~/dev",
                    desc: "Directory to search for projects in",
                },
                SubCommandArg {
                    alias: "a",
                    default: "",
                    desc: "Show all folders (including hidden ones)",
                },
            ]),
        }]);

        for subcommand in commands {
            println!("  {}\t\t{}", subcommand.alias, subcommand.desc);
            for argcommand in subcommand.args {
                let mut arg_name = argcommand.alias.to_owned();
                if argcommand.default.len() > 0 {
                    arg_name.push_str("=");
                    arg_name.push_str(argcommand.default.dimmed().to_string().as_str());
                }

                println!(
                    "  \t-{}\t{}",
                    arg_name,
                    argcommand.desc.to_string().as_str()
                )
            }
        }

        println!("{} {}", "---- ".dimmed(), "LTools".yellow());

        return;
    }

    let query = &args[1];
    println!("{}", query);
}
