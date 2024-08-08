//! Generates man pages and shell completions for hyprnome.

include!("src/cli.rs");

use clap::Command;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use clap_mangen::Man;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

static NAME: &str = "hyprnome";

fn generate_man_pages(cmd: Command) -> Result<(), Box<dyn Error>> {
    let man_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/man");
    let mut buffer = Vec::default();

    Man::new(cmd).render(&mut buffer)?;
    fs::create_dir_all(&man_dir)?;
    fs::write(man_dir.join(NAME.to_owned() + ".1"), buffer)?;

    Ok(())
}

fn generate_shell_completions(mut cmd: Command) -> Result<(), Box<dyn Error>> {
    let comp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/completions");

    fs::create_dir_all(&comp_dir)?;

    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, NAME, &comp_dir)?;
    }

    Ok(())
}

fn main() {
    let mut cmd = Cli::command();
    cmd.set_bin_name(NAME);

    if let Err(err) = generate_man_pages(cmd.clone()) {
        println!("cargo::warning=Error generating man pages: {err}");
    }

    if let Err(err) = generate_shell_completions(cmd) {
        println!("cargo::warning=Error generating shell completions: {err}");
    }
}
