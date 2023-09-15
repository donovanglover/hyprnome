use clap::Parser;
use cli::Cli;
use hyprland::data::Monitors;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::prelude::*;

mod cli;

/// Log information with --verbose
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}

/// Struct to keep related workspace state together
#[derive(Default)]
pub struct WorkspaceState {
    pub current_id: i32,
    pub monitor_ids: Vec<i32>,
    pub occupied_ids: Vec<i32>,
}

/// Implementation for WorkspaceState
impl WorkspaceState {
    /// Creates a new WorkspaceState
    pub fn new() -> WorkspaceState {
        let workspaces = Workspaces::get().unwrap();
        let mut monitor_ids: Vec<i32> = workspaces
            .clone()
            .filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name)
            .map(|x| x.id)
            .filter(|x| x > &0)
            .collect();
        let mut occupied_ids: Vec<i32> = workspaces.map(|x| x.id).filter(|x| x > &0).collect();

        monitor_ids.sort();
        occupied_ids.sort();

        WorkspaceState {
            current_id: Workspace::get_active().unwrap().id,
            monitor_ids,
            occupied_ids,
        }
    }

    /// Logs the current WorkspaceState
    pub fn log(&self) {
        log(&format!("Current ID:\t{}", self.current_id));
        log(&format!("Monitor IDs:\t{:?}", self.monitor_ids));
        log(&format!("Occupied IDs:\t{:?}", self.occupied_ids));
    }
}

/// Gets the previous workspace on a monitor, or try to choose the next left-most empty workspace
///
/// 1) If we're the first workspace on a monitor:
///     1.1) If we're at the lowest possible id 1 or the user doesn't want empty workspaces, return the current id
///     1.2) Otherwise, return the first unoccupied workspace before the current id
///         1.2.1) If all other workspaces before are occupied, return the current id instead
/// 2) Otherwise, since there are workspaces before on the same monitor, select the one before.
pub fn get_previous_id(state: WorkspaceState, no_empty_before: bool) -> i32 {
    let WorkspaceState {
        current_id,
        monitor_ids,
        occupied_ids,
    } = state;

    if monitor_ids[0] == current_id {
        if monitor_ids[0] == 1 || no_empty_before {
            current_id
        } else {
            let mut i = current_id - 1;

            while i > 0 {
                if !occupied_ids.contains(&i) {
                    return i;
                }

                i -= 1;
            }

            current_id
        }
    } else {
        monitor_ids[monitor_ids.iter().position(|&x| x == current_id).unwrap() - 1]
    }
}

/// Gets the next workspace on a monitor, or try to choose the next right-most empty workspace
///
/// 1) If we're the last workspace on a monitor:
///     1.1) If we're at the MAX or the user doesn't want empty workspaces, return the current id
///     1.2) Otherwise, return the first unoccupied workspace after the current id
///         1.2.1) If all other workspaces after are occupied, return the current id instead
/// 2) Otherwise, since there are workspaces after on the same monitor, select the one after.
pub fn get_next_id(state: WorkspaceState, no_empty_after: bool) -> i32 {
    let WorkspaceState {
        current_id,
        monitor_ids,
        occupied_ids,
    } = state;

    if monitor_ids[monitor_ids.len() - 1] == current_id {
        if monitor_ids[monitor_ids.len() - 1] == i32::MAX || no_empty_after {
            current_id
        } else {
            let mut i = current_id + 1;

            while i < i32::MAX {
                if !occupied_ids.contains(&i) {
                    return i;
                }

                i += 1;
            }

            if !occupied_ids.contains(&i) { i } else { current_id }
        }
    } else {
        monitor_ids[monitor_ids.iter().position(|&x| x == current_id).unwrap() + 1]
    }
}

/// Gets an ID to dispatch based on the current workspace state and cli options
pub fn get_id() -> i32 {
    let state = WorkspaceState::new();
    let Cli {
        previous,
        no_empty_before,
        no_empty_after,
        ..
    } = Cli::parse();

    state.log();

    if previous {
        get_previous_id(state, no_empty_before)
    } else {
        get_next_id(state, no_empty_after)
    }
}
