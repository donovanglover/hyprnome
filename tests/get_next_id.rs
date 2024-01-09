#![cfg_attr(rustfmt, rustfmt_skip)]

use hyprnome::WorkspaceState;

#[test]
fn only_workspace() {
    let current_id = 500;
    let monitor_ids = [500].to_vec();
    let occupied_ids = [500].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 501, "should return next workspace on monitor when only workspace");

    let current_id = 498;
    let monitor_ids = [498].to_vec();
    let occupied_ids = [498].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_after(true);

    assert_eq!(state.get_next_id(), 498, "should return the same workspace if only workspace and no_empty_after");
}

#[test]
fn next_workspace_on_monitor() {
    let current_id = 500;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 502, "should return next workspace on monitor");

    let current_id = 500;
    let monitor_ids = [500, 504].to_vec();
    let occupied_ids = [500, 502, 504].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 504, "should return next workspace on monitor with occupied workspaces in-between on other monitors");
}

#[test]
fn next_empty() {
    let current_id = 502;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 503, "should return next empty workspace if last workspace on monitor");

    let current_id = 502;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502, 503, 504].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 505, "should return next empty workspace if last workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn no_empty_after() {
    let current_id = 502;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_after(true);

    assert_eq!(state.get_next_id(), 502, "should return the same workspace if last workspace on monitor");

    let current_id = 502;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502, 503, 504].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_after(true);

    assert_eq!(state.get_next_id(), 502, "should return the same workspace if last workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn out_of_bounds() {
    let current_id = i32::MAX;
    let monitor_ids = [500, i32::MAX].to_vec();
    let occupied_ids = [500, i32::MAX].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), i32::MAX, "should return the same workspace if last workspace is i32::MAX");

    let current_id = i32::MAX - 2;
    let monitor_ids = [i32::MAX - 2].to_vec();
    let occupied_ids = [i32::MAX - 2, i32::MAX - 1, i32::MAX].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), i32::MAX - 2, "should return the current workspace if all next workspaces are occupied");
}

#[test]
fn fill_the_gaps() {
    let current_id = 4;
    let monitor_ids = [3, 4].to_vec();
    let occupied_ids = [3, 4, 6].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 5, "should return workspace 5 if the occupied workspaces are [3, 4, 6], the monitor workspaces are [3, 4], and the current workspace is 4");

    let current_id = 5;
    let monitor_ids = [4, 5].to_vec();
    let occupied_ids = [4, 5, 6, 7, 9].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), 8, "should return workspace 8 if the occupied workspaces are [4, 5, 6, 7, 9], the monitor workspaces are [4, 5], and the current workspace is 5");
}

#[test]
fn returns_the_last_id() {
    let current_id = i32::MAX - 1;
    let monitor_ids = [i32::MAX - 1].to_vec();
    let occupied_ids = [i32::MAX - 1].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), i32::MAX, "should return i32::MAX if it's unoccupied");

    let current_id = i32::MAX - 2;
    let monitor_ids = [i32::MAX - 2].to_vec();
    let occupied_ids = [i32::MAX - 2, i32::MAX - 1].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_next_id(), i32::MAX, "should return i32::MAX if it's unoccupied and there are in-between workspaces on other monitors");
}
