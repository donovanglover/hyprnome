use clap::Parser;
use cli::Cli;
use hyprland::dispatch::*;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial as WID;
use hyprnome::get_id;
// use hyprnome::log;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() -> hyprland::Result<()> {
    let Cli { previous, .. } = Cli::parse();
    let new_id = get_id(previous);
    hyprland::dispatch!(Workspace, WID::Id(new_id))?;

    if previous {
        hyprland::dispatch!(Workspace, WID::RelativeMonitor(-1))?;
    } else {
        hyprland::dispatch!(Workspace, WID::RelativeMonitor(1))?;
    }

    Ok(())
}
