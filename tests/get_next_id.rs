#![cfg_attr(rustfmt, rustfmt_skip)]

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

#[test]
fn fill_the_gaps() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: 4,
        monitor_ids: [3, 4].to_vec(),
        occupied_ids: [3, 4, 6].to_vec(),
    }, false), 5, "should return workspace 5 if the occupied workspaces are [3, 4, 6], the monitor workspaces are [3, 4], and the current workspace is 4");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: 5,
        monitor_ids: [4, 5].to_vec(),
        occupied_ids: [4, 5, 6, 7, 9].to_vec(),
    }, false), 8, "should return workspace 8 if the occupied workspaces are [4, 5, 6, 7, 9], the monitor workspaces are [4, 5], and the current workspace is 5");
}

#[test]
fn returns_the_last_id() {
    assert_eq!(get_next_id(WorkspaceState {
        current_id: i32::MAX - 1,
        monitor_ids: [i32::MAX - 1].to_vec(),
        occupied_ids: [i32::MAX - 1].to_vec(),
    }, false), i32::MAX, "should return i32::MAX if it's unoccupied");

    assert_eq!(get_next_id(WorkspaceState {
        current_id: i32::MAX - 2,
        monitor_ids: [i32::MAX - 2].to_vec(),
        occupied_ids: [i32::MAX - 2, i32::MAX - 1].to_vec(),
    }, false), i32::MAX, "should return i32::MAX if it's unoccupied and there are in-between workspaces on other monitors");
}
