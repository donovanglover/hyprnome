#![doc = include_str!("../README.md")]

mod cli;
mod hyprland;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let (_move, keep_special, previous, no_empty, no_empty_before, no_empty_after) = cli::get_options();
    let mut state = hyprland::get_state();

    state.set_no_empty_before(no_empty || no_empty_before);
    state.set_no_empty_after(no_empty || no_empty_after);
    state.set_previous(previous);

    cli::log(&format!("{}", state));

    let id = state.get_id();

    if hyprland::change_workspace(id, _move, keep_special).is_ok() {
        cli::log(&format!("Dispatched ID:\t{id}"));
    } else {
        cli::log(&format!("Failed to dispatch ID {id}"));
    }
}
