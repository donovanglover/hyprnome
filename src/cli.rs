use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

const LONG_ABOUT: &str = "
hyprnome is a program that enables you to achieve GNOME-like workspace switching in Hyprland.

Unlike Hyprland's r+1/r-1, empty workspaces between two occupied workspaces are skipped.

Unlike Hyprland's m+1/m-1, new workspaces are created instead of wrapping to existing ones.

Example hyprland.conf:

bind = $SUPER, 1, exec, hyprnome --previous

bind = $SUPER, 2, exec, hyprnome

bind = $SUPER_SHIFT, 1, exec, hyprnome --previous --move

bind = $SUPER_SHIFT, 2, exec, hyprnome --move
";

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

    /// Don't create empty workspaces to the left
    #[arg(short, long, default_value_t = false)]
    pub no_empty_before: bool,

    /// Don't create empty workspaces to the right
    #[arg(short = 'N', long, default_value_t = false)]
    pub no_empty_after: bool,

    /// Move the active window to the dispatched workspace
    #[arg(short, long, default_value_t = false)]
    pub _move: bool,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
