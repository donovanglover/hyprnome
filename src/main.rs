use clap::Parser;
use cli::Cli;
use hyprland::dispatch::*;
use hyprnome::get_id;
use hyprnome::is_special;
use hyprnome::log;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() -> hyprland::Result<()> {
    let Cli {
        _move,
        keep_special,
        ..
    } = Cli::parse();

    let id = WorkspaceIdentifierWithSpecial::Id(get_id());

    log(&format!("Dispatched ID:\t{id}"));

    if _move {
        let was_special = is_special();

        hyprland::dispatch!(MoveToWorkspace, id, None)?;

        if !keep_special && was_special {
            hyprland::dispatch!(ToggleSpecialWorkspace, None)
        } else {
            Ok(())
        }
    } else {
        hyprland::dispatch!(Workspace, id)
    }
}
