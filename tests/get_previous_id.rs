#![cfg_attr(rustfmt, rustfmt_skip)]

use hyprnome::WorkspaceState;

#[test]
fn only_workspace() {
    let current_id = 500;
    let monitor_ids = [500].to_vec();
    let occupied_ids = [500].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 499, "should return previous workspace on monitor when only workspace");

    let current_id = 500;
    let monitor_ids = [500].to_vec();
    let occupied_ids = [500].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_before(true);

    assert_eq!(state.get_previous_id(), 500, "should return the same workspace if only workspace and no_empty_before");
}

#[test]
fn previous_workspace_on_monitor() {
    let current_id = 502;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 500, "should return previous workspace on monitor");

    let current_id = 504;
    let monitor_ids = [500, 504].to_vec();
    let occupied_ids = [500, 502, 504].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 500, "should return previous workspace on monitor with occupied workspaces in-between on other monitors");
}

#[test]
fn previous_empty() {
    let current_id = 500;
    let monitor_ids = [500, 501].to_vec();
    let occupied_ids = [500, 501].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 499, "should return previous empty workspace if first workspace on monitor");

    let current_id = 500;
    let monitor_ids = [500, 501].to_vec();
    let occupied_ids = [499, 500, 501].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 498, "should return previous empty workspace if first workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn no_empty_before() {
    let current_id = 500;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [500, 502].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_before(true);

    assert_eq!(state.get_previous_id(), 500, "should return the same workspace if first workspace on monitor");

    let current_id = 500;
    let monitor_ids = [500, 502].to_vec();
    let occupied_ids = [498, 499, 500, 502].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_no_empty_before(true);

    assert_eq!(state.get_previous_id(), 500, "should return the same workspace if first workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn out_of_bounds() {
    let current_id = 1;
    let monitor_ids = [1, 2].to_vec();
    let occupied_ids = [1, 2].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 1, "should return the same workspace if first workspace is 1");

    let current_id = 3;
    let monitor_ids = [3, 4].to_vec();
    let occupied_ids = [1, 2, 3, 4].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 3, "should return the current workspace if all previous workspaces are occupied");
}

#[test]
fn fill_the_gaps() {
    let current_id = 3;
    let monitor_ids = [3, 4].to_vec();
    let occupied_ids = [1, 3, 4].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 2, "should return workspace 2 if the occupied workspaces are [1, 3, 4], the monitor workspaces are [3, 4], and the current workspace is 3");

    let current_id = 4;
    let monitor_ids = [4, 5].to_vec();
    let occupied_ids = [1, 3, 4, 5].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 2, "should return workspace 2 if the occupied workspaces are [1, 3, 4, 5], the monitor workspaces are [4, 5], and the current workspace is 4");
}

#[test]
fn returns_the_first_id() {
    let current_id = 2;
    let monitor_ids = [2, 3].to_vec();
    let occupied_ids = [2, 3].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 1, "should return 1 if it's unoccupied");

    let current_id = 4;
    let monitor_ids = [4, 5].to_vec();
    let occupied_ids = [2, 3, 4, 5].to_vec();
    let state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    assert_eq!(state.get_previous_id(), 1, "should return 1 if it's unoccupied and there are in-between workspaces on other monitors");
}

#[test]
fn cycle() {
    let current_id = 500;
    let monitor_ids = [500, 502, 504].to_vec();
    let occupied_ids = [498, 500, 502, 504].to_vec();
    let mut state = WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    state.set_cycle(true);

    assert_eq!(state.get_previous_id(), 504, "should return the last workspace on monitor if first workspace on monitor");
}
