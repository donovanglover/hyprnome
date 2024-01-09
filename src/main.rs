#![doc = include_str!("../README.md")]

mod hyprland;
mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let _move = cli::get_move();
    let keep_special = cli::get_keep_special();

    let (current_id, monitor_ids, occupied_ids) = hyprland::get_state();
    let workspace_state = hyprnome::WorkspaceState::new(current_id, monitor_ids, occupied_ids);

    cli::log(&format!("{}", workspace_state));

    let (previous, no_empty, no_empty_before, no_empty_after) = cli::get_options();

    let id = if previous {
        workspace_state.get_previous_id(no_empty || no_empty_before)
    } else {
        workspace_state.get_next_id(no_empty || no_empty_after)
    };

    cli::log(&format!("Dispatched ID:\t{id}"));

    let _ = hyprland::change_workspace(id, _move, keep_special);
}
