use clap::Parser;
use cli::Cli;
use hyprland::dispatch::*;
use hyprnome::get_id;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let Cli { previous, _move, .. } = Cli::parse();

    if _move {
        hyprland::dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(get_id(previous)), None).unwrap();
    } else {
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(get_id(previous))).unwrap();
    }
}
