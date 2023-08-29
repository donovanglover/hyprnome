use clap::Parser;
use cli::Cli;
use hyprnome::log;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let Cli {
        allow_going_backwards,
        ..
    } = Cli::parse();

    log(&format!("{allow_going_backwards}"));
}
