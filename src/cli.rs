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
pub struct Cli {
    /// Go to the previous workspace instead of the next
    #[arg(short, long, default_value_t = false)]
    pub previous: bool,

    /// Move the active window to the dispatched workspace
    #[arg(short, long, default_value_t = false)]
    pub _move: bool,

    /// Don't create empty workspaces when reaching the start/end
    #[arg(short, long, default_value_t = false)]
    pub no_empty: bool,

    /// Don't auto-close special workspaces when switching workspaces
    ///
    /// With --move:
    ///
    /// Hyprland v0.29.0 and above allow opening empty special workspaces. This changed
    /// the behavior of special workspaces, most notably not auto-closing them by default
    /// when moving windows outside them.
    ///
    /// hyprdim v0.2.0 auto-closes special workspaces by default to mimic the old behavior.
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
    #[arg(short, long, default_value_t = false)]
    pub keep_special: bool,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
