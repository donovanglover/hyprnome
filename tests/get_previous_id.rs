use hyprnome::WorkspaceState;
use hyprnome::get_previous_id;

#[test]
fn only_workspace() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500].to_vec(),
        occupied_ids: [500].to_vec(),
    }, false), 499, "should return previous workspace on monitor when only workspace");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500].to_vec(),
        occupied_ids: [500].to_vec(),
    }, true), 500, "should return the same workspace if only workspace and no_empty_before");
}

#[test]
fn next_workspace_on_monitor() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 502,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502].to_vec(),
    }, false), 500, "should return previous workspace on monitor");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 504,
        monitor_ids: [500, 504].to_vec(),
        occupied_ids: [500, 502, 504].to_vec(),
    }, false), 500, "should return previous workspace on monitor with occupied workspaces in-between on other monitors");
}

#[test]
fn next_empty() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 501].to_vec(),
        occupied_ids: [500, 501].to_vec(),
    }, false), 499, "should return previous empty workspace if first workspace on monitor");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 501].to_vec(),
        occupied_ids: [499, 500, 501].to_vec(),
    }, false), 498, "should return previous empty workspace if first workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn no_empty_before() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [500, 502].to_vec(),
    }, true), 500, "should return the same workspace if first workspace on monitor");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 500,
        monitor_ids: [500, 502].to_vec(),
        occupied_ids: [498, 499, 500, 502].to_vec(),
    }, true), 500, "should return the same workspace if first workspace on monitor with occupied workspaces on other monitors");
}

#[test]
fn out_of_bounds() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 1,
        monitor_ids: [1, 2].to_vec(),
        occupied_ids: [1, 2].to_vec(),
    }, false), 1, "should return the same workspace if first workspace is 1");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 3,
        monitor_ids: [3, 4].to_vec(),
        occupied_ids: [1, 2, 3, 4].to_vec(),
    }, false), 3, "should return the current workspace if all previous workspaces are occupied");
}

#[test]
fn fill_the_gaps() {
    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 3,
        monitor_ids: [3, 4].to_vec(),
        occupied_ids: [1, 3, 4].to_vec(),
    }, false), 2, "should return workspace 2 if the occupied workspaces are [1, 3, 4], the monitor workspaces are [3, 4], and the current workspace is 3");

    assert_eq!(get_previous_id(WorkspaceState {
        current_id: 4,
        monitor_ids: [4, 5].to_vec(),
        occupied_ids: [1, 3, 4, 5].to_vec(),
    }, false), 2, "should return workspace 2 if the occupied workspaces are [1, 3, 4, 5], the monitor workspaces are [4, 5], and the current workspace is 4");
}
