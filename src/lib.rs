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

#[derive(Default)]
pub struct WorkspaceState {
    pub current_id: i32,
    pub monitor_ids: Vec<i32>,
    pub occupied_ids: Vec<i32>
}

impl WorkspaceState {
    pub fn new() -> WorkspaceState {
        let workspaces = Workspaces::get().unwrap();
        let mut monitor_ids: Vec<i32> = workspaces.clone().filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name).map(|x| x.id).filter(|x| x > &0).collect();
        let mut occupied_ids: Vec<i32> = workspaces.map(|x| x.id).filter(|x| x > &0).collect();

        monitor_ids.sort();
        occupied_ids.sort();

        WorkspaceState {
            current_id: Workspace::get_active().unwrap().id,
            monitor_ids,
            occupied_ids,
        }
    }
}

/// Gets the previous workspace on a monitor, or try to choose a new left-most empty workspace
///
/// 1) Returns the workspace id before the first occupied workspace (or 1)
/// 2) Otherwise, since there are workspaces before on the same monitor, select the one before.
pub fn get_previous_id(state: WorkspaceState) -> i32 {
    let WorkspaceState { current_id, monitor_ids, occupied_ids } = state;

    if monitor_ids[0] == current_id {
        if occupied_ids[0] == 1 { 1 } else { occupied_ids[0] - 1 }
    } else {
        monitor_ids[monitor_ids.iter().position(|&x| x == current_id).unwrap() - 1]
    }
}

/// Gets the next workspace on a monitor, or choose a new right-most empty workspace
///
/// 1) Returns the workspace id after the last occupied workspace
/// 2) Otherwise, since there are workspaces after on the same monitor, select the one after
pub fn get_next_id(state: WorkspaceState) -> i32 {
    let WorkspaceState { current_id, monitor_ids, occupied_ids } = state;

    if monitor_ids[monitor_ids.len() - 1] == current_id {
        occupied_ids[occupied_ids.len() - 1] + 1
    } else {
        monitor_ids[monitor_ids.iter().position(|&x| x == current_id).unwrap() + 1]
    }
}

pub fn get_id(previous: bool) -> i32 {
    let state = WorkspaceState::new();

    if previous { get_previous_id(state) } else { get_next_id(state) }
}
