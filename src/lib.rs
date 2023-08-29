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

/// Gets the id
pub fn get_id(state: WorkspaceState) -> i32 {
    let WorkspaceState { current_id, monitor_ids, occupied_ids } = state;
    // let current_id = 500
    // let occupied_ids = [495, 496, 498, 500, 502, 504, 505]
    // let monitor_ids = [496, 500, 504]
    // if current_id is_not_first_in(monitor_ids) -> RelativeMonitor -1
    // else new_workspace(first_id_of(occupied_ids) - 1)
    if monitor_ids[0] == current_id {
        let id = occupied_ids[0] - 1;
    }
    0
}

pub fn get_workspace_state() -> WorkspaceState {
    let workspaces = Workspaces::get().unwrap();

    WorkspaceState {
        current_id: Workspace::get_active().unwrap().id,
        monitor_ids: workspaces.clone().filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name).map(|x| x.id).collect(),
        occupied_ids: workspaces.map(|x| x.id).collect(),
    }
}
