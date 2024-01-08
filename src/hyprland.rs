use hyprland::data::Client;
use hyprland::data::Monitors;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::prelude::*;

fn new() {
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

    Self {
        current_id: Workspace::get_active().unwrap().id,
        monitor_ids,
        occupied_ids,
    }
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
