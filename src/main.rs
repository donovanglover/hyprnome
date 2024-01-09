mod hyprland;
mod cli;

/// Main function in charge of hyprnome logic.
///
/// Specific features are abstracted into lib to make things testable.
fn main() {
    let _move = cli::get_move();
    let keep_special = cli::get_keep_special();

    let state = hyprland::get_state();
    let workspace_state = hyprnome::WorkspaceState::new(state.0, state.1, state.2);

    cli::log(&format!("{}", workspace_state));

    let options = cli::get_options();

    let id = if options.0 {
        workspace_state.get_previous_id(options.1 || options.2)
    } else {
        workspace_state.get_next_id(options.1 || options.3)
    };

    cli::log(&format!("Dispatched ID:\t{id}"));

    let _ = hyprland::change_workspace(id, _move, keep_special);
}
