use clap::Parser;
use cli::Cli;

mod cli;

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}

/// Gets the id
pub fn get_id(current_id: i32, monitor_ids: Vec<i32>, occupied_ids: Vec<i32>) -> i32 {
    // let current_id = 500
    // let occupied_ids = [495, 496, 498, 500, 502, 504, 505]
    // let monitor_ids = [496, 500, 504]
    // if current_id is_not_first_in(monitor_ids) -> RelativeMonitor -1
    // else new_workspace(first_id_of(occupied_ids) - 1)
    if monitor_ids[0] == current_id {
        let id = occupied_ids[0] - 1;
    }
    0
}
