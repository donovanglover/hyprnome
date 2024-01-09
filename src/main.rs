#![doc = include_str!("../README.md")]

mod hyprland;
mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let (_move, keep_special, previous, no_empty, no_empty_before, no_empty_after) = cli::get_options();
    let workspace_state = hyprland::get_state();

    cli::log(&format!("{}", workspace_state));

    let id = if previous {
        workspace_state.get_previous_id(no_empty || no_empty_before)
    } else {
        workspace_state.get_next_id(no_empty || no_empty_after)
    };

    cli::log(&format!("Dispatched ID:\t{id}"));

    let _ = hyprland::change_workspace(id, _move, keep_special);
}
