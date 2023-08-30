use clap::Parser;
use cli::Cli;
use hyprland::dispatch::*;
use hyprnome::get_id;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() -> hyprland::Result<()> {
    let Cli { _move, .. } = Cli::parse();
    let id = WorkspaceIdentifierWithSpecial::Id(get_id());

    if _move {
        hyprland::dispatch!(MoveToWorkspace, id, None)
    } else {
        hyprland::dispatch!(Workspace, id)
    }
}
