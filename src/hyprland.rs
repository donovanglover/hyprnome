use hyprland::data::Client;
use hyprland::data::Monitors;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;
use hyprnome::WorkspaceState;

pub fn get_state() -> WorkspaceState {
    let workspaces = Workspaces::get().unwrap();
    let current_id = Workspace::get_active().unwrap().id;
    let monitor_ids: Vec<i32> = workspaces
        .clone()
        .filter(|x| x.monitor == Monitors::get().unwrap().find(|x| x.focused).unwrap().name)
        .map(|x| x.id)
        .filter(|x| x > &0)
        .collect();
    let occupied_ids: Vec<i32> = workspaces.map(|x| x.id).filter(|x| x > &0).collect();

    WorkspaceState::new(current_id, monitor_ids, occupied_ids)
}

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
///
/// The if statement is used to make sure this function works when no window
/// is the active window.
pub fn is_special() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        return workspace.name.contains("special");
    }

    false
}

pub fn change_workspace(id: i32, _move: bool, keep_special: bool) -> hyprland::Result<()> {
    let id = WorkspaceIdentifierWithSpecial::Id(id);

    if _move {
        let was_special = is_special();

        hyprland::dispatch!(MoveToWorkspace, id, None)?;

        if !keep_special && was_special {
            hyprland::dispatch!(ToggleSpecialWorkspace, None)
        } else {
            Ok(())
        }
    } else {
        if !keep_special && is_special() {
            hyprland::dispatch!(ToggleSpecialWorkspace, None)?;
        }

        hyprland::dispatch!(Workspace, id)
    }
}
