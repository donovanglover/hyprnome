use clap::Parser;
use cli::Cli;

mod cli;

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}
