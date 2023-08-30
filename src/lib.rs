use clap::Parser;
use cli::Cli;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::data::Monitors;
use hyprland::prelude::*;

mod cli;

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}

pub struct WorkspaceState {
    pub current_id: i32,
    pub monitor_ids: Vec<i32>,
    pub occupied_ids: Vec<i32>
}

/// Gets the previous workspace on a monitor, or try to choose a new left-most empty workspace
pub fn get_previous_id(state: WorkspaceState) -> i32 {
    let WorkspaceState { current_id, monitor_ids, occupied_ids } = state;

    // If the current workspace is the first workspace on the monitor
    if monitor_ids[0] == current_id {
        // Return the workspace id before the first occupied workspace
        //
        // Return workspace 1 if there's no workspace before to choose from
        //
        // This basically selects an empty workspace in the direction we want
        return if occupied_ids[0] == 1 { 1 } else { occupied_ids[0] - 1 };
    }

    // Otherwise, since there are workspaces before on the same monitor, select the one before.
    monitor_ids[monitor_ids.iter().position(|&x| x == current_id).unwrap() - 1]
}

pub fn get_workspace_state() -> WorkspaceState {
    let workspaces = Workspaces::get().unwrap();

    WorkspaceState {
        current_id: Workspace::get_active().unwrap().id,
        monitor_ids: workspaces.clone().filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name).map(|x| x.id).collect(),
        occupied_ids: workspaces.map(|x| x.id).collect(),
    }
}
