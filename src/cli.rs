use clap::Parser;

const LONG_ABOUT: &str = "
hyprnome is a program that enables you to achieve GNOME-like workspace switching in Hyprland.

Unlike Hyprland's r+1/r-1, empty workspaces are not shown to the user when the user navigates
between them.
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    /// Go to the previous workspace
    #[arg(short, long, default_value_t = false)]
    pub previous: bool,

    /// Create empty workspaces when going backwards
    #[arg(short, long, default_value_t = false)]
    pub allow_going_backwards: bool,

    /// Move the selected window
    #[arg(short, long, default_value_t = false)]
    pub _move: bool,

    /// Show information about what hyprnome is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
