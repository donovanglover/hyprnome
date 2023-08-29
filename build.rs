include!("src/cli.rs");

use clap::Command;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use clap_mangen::Man;

static NAME: &str = "hyprnome";

fn generate_man_pages(cmd: Command) {
    let man_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("man");

    std::fs::create_dir_all(&man_dir).unwrap();

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).expect("Man page generation failed");
    std::fs::write(man_dir.join(NAME.to_owned() + ".1"), buffer).expect("Failed to write man page");
}

fn generate_shell_completions(mut cmd: Command) {
    let comp_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("completions");

    std::fs::create_dir_all(&comp_dir).unwrap();

    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, NAME, &comp_dir).unwrap();
    }
}

fn main() {
    let mut cmd = Cli::command();
    cmd.set_bin_name(NAME);

    generate_man_pages(cmd.clone());
    generate_shell_completions(cmd);
}
