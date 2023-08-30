use clap::Parser;

const LONG_ABOUT: &str = "
hyprnome is a program that enables you to achieve GNOME-like workspace switching in Hyprland.

Unlike Hyprland's r+1/r-1, empty workspaces are not shown to the user when the user navigates
between them.
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
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
