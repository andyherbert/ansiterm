use clap::Parser;
mod sauce_cmds;
use sauce_cmds::SauceCommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Commands {
    /// Various commands to manipulate SAUCE records
    #[clap(subcommand)]
    Sauce(SauceCommand),
}

fn main() {
    match Commands::parse() {
        Commands::Sauce(sauce_cmd) => sauce_cmds::main(sauce_cmd),
    }
}
