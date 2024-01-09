use hyprland::data::Client;
use hyprland::data::Monitors;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;
use hyprnome::WorkspaceState;

pub fn get_state() -> hyprland::Result<WorkspaceState> {
    let monitors = Monitors::get()?;
    let workspaces = Workspaces::get()?;
    let current_id = Workspace::get_active()?.id;

    let monitor_ids: Vec<i32> = workspaces
        .clone()
        .filter(|workspace| {
            if let Some(monitor) = monitors.clone().find(|monitor| monitor.focused) {
                workspace.monitor == monitor.name
            } else {
                false
            }
        })
        .map(|workspace| workspace.id)
        .filter(|workspace| *workspace > 0)
        .collect();

    let occupied_ids: Vec<i32> = workspaces
        .map(|workspace| workspace.id)
        .filter(|workspace| *workspace > 0)
        .collect();

    Ok(WorkspaceState::new(current_id, monitor_ids, occupied_ids))
}

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
///
/// The if statement is used to make sure this function works when no window
/// is the active window.
pub fn is_special() -> hyprland::Result<bool> {
    if let Some(client) = Client::get_active()? {
        let Client { workspace, .. } = client;

        return Ok(workspace.name.contains("special"));
    }

    Ok(false)
}

pub fn change_workspace(id: i32, _move: bool, keep_special: bool) -> hyprland::Result<()> {
    let id = WorkspaceIdentifierWithSpecial::Id(id);

    if _move {
        let was_special = is_special()?;

        hyprland::dispatch!(MoveToWorkspace, id, None)?;

        if !keep_special && was_special {
            hyprland::dispatch!(ToggleSpecialWorkspace, None)
        } else {
            Ok(())
        }
    } else {
        if !keep_special && is_special()? {
            hyprland::dispatch!(ToggleSpecialWorkspace, None)?;
        }

        hyprland::dispatch!(Workspace, id)
    }
}
