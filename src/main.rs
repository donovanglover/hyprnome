use clap::Parser;
use cli::Cli;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::data::Monitors;
use hyprland::dispatch::*;
use hyprland::prelude::*;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial as WID;
use hyprnome::get_id;
// use hyprnome::log;

mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() -> hyprland::Result<()> {
    let Cli { previous, .. } = Cli::parse();
    let workspaces = Workspaces::get()?;

    let current_id = Workspace::get_active()?.id;
    let monitor_ids: Vec<i32> = workspaces.clone().filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name).map(|x| x.id).collect();
    let occupied_ids: Vec<i32> = workspaces.map(|x| x.id).collect();

    let new_id = get_id(current_id, monitor_ids, occupied_ids);
    hyprland::dispatch!(Workspace, WID::Id(new_id))?;

    if previous {
        hyprland::dispatch!(Workspace, WID::RelativeMonitor(-1))?;
    } else {
        hyprland::dispatch!(Workspace, WID::RelativeMonitor(1))?;
    }

    Ok(())
}
