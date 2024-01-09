use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

const LONG_ABOUT: &str = "
hyprnome is a program that enables you to achieve GNOME-like workspace switching in Hyprland.

Unlike Hyprland's r+1/r-1, empty workspaces between two occupied workspaces are skipped.

Unlike Hyprland's m+1/m-1, new workspaces are created instead of wrapping to existing ones.

Example hyprland.conf:

bind = SUPER, 1, exec, hyprnome --previous

bind = SUPER, 2, exec, hyprnome

bind = SUPER_SHIFT, 1, exec, hyprnome --previous --move

bind = SUPER_SHIFT, 2, exec, hyprnome --move

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.";

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT, styles = styles())]
struct Cli {
    /// Go to the previous workspace instead of the next
    ///
    /// By default hyprnome will advance to the next workspace. --previous is necessary
    /// when you want to go backwards in the list of occupied workspaces.
    #[arg(short, long, default_value_t = false)]
    previous: bool,

    /// Move the active window to the dispatched workspace
    ///
    /// This flag lets you move windows across workspaces without having to worry about
    /// the current workspace you're in.
    #[arg(short, long, default_value_t = false)]
    _move: bool,

    /// Don't create empty workspaces in the given direction
    ///
    /// This prevents empty workspaces from being created when no occupied workspaces
    /// remain in the given direction.
    #[arg(short, long, default_value_t = false)]
    no_empty: bool,

    /// Don't create empty workspaces to the left
    ///
    /// NOTE: This flag is deprecated and has been replaced with --no-empty. The flag is
    /// kept for backwards compatibility with v0.1.0, but is hidden by default.
    #[arg(long, default_value_t = false, hide = true)]
    no_empty_before: bool,

    /// Don't create empty workspaces to the right
    ///
    /// NOTE: This flag is deprecated and has been replaced with --no-empty. The flag is
    /// kept for backwards compatibility with v0.1.0, but is hidden by default.
    #[arg(short = 'N', long, default_value_t = false, hide = true)]
    no_empty_after: bool,

    /// Don't auto-close special workspaces when switching workspaces
    ///
    /// With --move:
    ///
    /// Hyprland v0.29.0 and above allow opening empty special workspaces. This changed
    /// the behavior of special workspaces, most notably not auto-closing them by default
    /// when moving windows outside them.
    ///
    /// hyprnome v0.2.0 auto-closes special workspaces by default to mimic the old behavior.
    /// In order to avoid breaking people's workflows, it's possible to keep the special workspace
    /// open with this flag, although you'd then have to manually close the special workspace.
    ///
    /// Without --move:
    ///
    /// Special workspaces are cool when used with translucent or non-opaque windows that let you
    /// see the workspaces behind them. They become problematic when there's an opaque window
    /// preventing you from seeing the workspaces you're switching to.
    ///
    /// By default when switching workspaces, any windows in the workspace behind are focused. To
    /// avoid accidentally thinking that the special workspace is focused, hyprnome will automatically
    /// close special workspaces by default when switching between non-special workspaces.
    ///
    /// This flag makes it possible to keep the special workspace visible when switching workspaces,
    /// although it won't be focused if the workspace behind it has one or more windows.
    ///
    /// Summary: This flag is available so users can choose the old behavior, however automatically
    /// closing special workspaces (the default) does have its benefits.
    #[arg(short, long, default_value_t = false)]
    keep_special: bool,

    /// Cycle between workspaces instead of creating new ones
    ///
    /// This flag makes it possible to cycle to the last workspace on a monitor when there are no
    /// other workspaces before, and to cycle to the first workspace on a monitor when there are no
    /// other workspaces after.
    #[arg(short, long, default_value_t = false)]
    cycle: bool,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

/// Log information with --verbose
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}");
    }
}

/// Gets an ID to dispatch based on the current workspace state and cli options
pub fn get_options() -> [bool; 7] {
    let Cli {
        _move,
        keep_special,
        previous,
        no_empty,
        no_empty_before,
        no_empty_after,
        cycle,
        ..
    } = Cli::parse();

    [_move, keep_special, previous, no_empty, no_empty_before, no_empty_after, cycle]
}
