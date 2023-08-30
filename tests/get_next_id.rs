use hyprnome::WorkspaceState;
use hyprnome::get_next_id;

#[test]
fn only_workspace() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500].to_vec(),
        occupied_ids: [500].to_vec(),
    }, false), 501, "should return next workspace on monitor when only workspace");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500].to_vec(),
        occupied_ids: [500].to_vec(),
    }, true), 500, "should return the same workspace if only workspace and no_empty_after");
}

#[test]
fn next_workspace_on_monitor() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502].to_vec(),
    }, false), 502, "should return next workspace on monitor");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 504].to_vec(),
        occupied_ids: [500, 502, 504].to_vec(),
    }, false), 504, "should return next workspace on monitor with occupied workspaces in-between on other monitors");
}

#[test]
fn next_empty() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: 502,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502].to_vec(),
    }, false), 503, "should return next empty workspace if last workspace on monitor");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: 502,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502, 503, 504].to_vec(),
    }, false), 505, "should return next empty workspace if last workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn no_empty_after() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: 502,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502].to_vec(),
    }, true), 502, "should return the same workspace if last workspace on monitor");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: 502,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502, 503, 504].to_vec(),
    }, true), 502, "should return the same workspace if last workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn out_of_bounds() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: i32::MAX,
        monitor_ids: [500, i32::MAX].to_vec(),
        occupied_ids: [500, i32::MAX].to_vec(),
    }, false), i32::MAX, "should return the same workspace if last workspace is i32::MAX");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: i32::MAX - 2,
        monitor_ids: [i32::MAX - 2].to_vec(),
        occupied_ids: [i32::MAX - 2, i32::MAX - 1, i32::MAX].to_vec(),
    }, false), i32::MAX - 2, "should return the current workspace if all next workspaces are occupied");
}
