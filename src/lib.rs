//! Library that can be used to get previous/next workspaces from a given state.

use std::fmt;

/// Struct to keep related workspace state together
#[derive(Default)]
pub struct WorkspaceState {
    current_id: i32,
    monitor_ids: Vec<i32>,
    occupied_ids: Vec<i32>,
    no_empty_before: bool,
    no_empty_after: bool,
    previous: bool,
}

/// Implementation for WorkspaceState
impl WorkspaceState {
    /// Creates a new WorkspaceState
    ///
    /// Vectors are sorted so it's easier to perform operations on them.
    pub fn new(current_id: i32, mut monitor_ids: Vec<i32>, mut occupied_ids: Vec<i32>) -> Self {
        monitor_ids.sort_unstable();
        occupied_ids.sort_unstable();

        Self {
            current_id,
            monitor_ids,
            occupied_ids,
            no_empty_before: false,
            no_empty_after: false,
            previous: false,
        }
    }

    /// Gets the previous workspace on a monitor, or try to choose the next left-most empty workspace
    ///
    /// 1) If we're the first workspace on a monitor:
    ///     1.1) If we're at the lowest possible id 1 or the user doesn't want empty workspaces, return the current id
    ///     1.2) Otherwise, return the first unoccupied workspace before the current id
    ///         1.2.1) If all other workspaces before are occupied, return the current id instead
    /// 2) Otherwise, since there are workspaces before on the same monitor, select the one before.
    pub fn get_previous_id(&self) -> i32 {
        if self.monitor_ids[0] == self.current_id {
            if self.monitor_ids[0] == 1 || self.no_empty_before {
                self.current_id
            } else {
                let mut i = self.current_id - 1;

                while i > 0 {
                    if !self.occupied_ids.contains(&i) {
                        return i;
                    }

                    i -= 1;
                }

                self.current_id
            }
        } else {
            self.monitor_ids[self.monitor_ids.iter().position(|&x| x == self.current_id).unwrap() - 1]
        }
    }

    /// Gets the next workspace on a monitor, or try to choose the next right-most empty workspace
    ///
    /// 1) If we're the last workspace on a monitor:
    ///     1.1) If we're at the MAX or the user doesn't want empty workspaces, return the current id
    ///     1.2) Otherwise, return the first unoccupied workspace after the current id
    ///         1.2.1) If all other workspaces after are occupied, return the current id instead
    /// 2) Otherwise, since there are workspaces after on the same monitor, select the one after.
    pub fn get_next_id(&self) -> i32 {
        if self.monitor_ids[self.monitor_ids.len() - 1] == self.current_id {
            if self.monitor_ids[self.monitor_ids.len() - 1] == i32::MAX || self.no_empty_after {
                self.current_id
            } else {
                let mut i = self.current_id + 1;

                while i < i32::MAX {
                    if !self.occupied_ids.contains(&i) {
                        return i;
                    }

                    i += 1;
                }

                if !self.occupied_ids.contains(&i) {
                    i
                } else {
                    self.current_id
                }
            }
        } else {
            self.monitor_ids[self.monitor_ids.iter().position(|&x| x == self.current_id).unwrap() + 1]
        }
    }

    /// Sets `no_empty_before`
    pub fn set_no_empty_before(&mut self, no_empty_before: bool) {
        self.no_empty_before = no_empty_before;
    }

    /// Sets `no_empty_after`
    pub fn set_no_empty_after(&mut self, no_empty_after: bool) {
        self.no_empty_after = no_empty_after;
    }

    /// Sets `previous`
    pub fn set_previous(&mut self, previous: bool) {
        self.previous = previous;
    }

    /// Gets an id based on the current state
    pub fn get_id(&self) -> i32 {
        if self.previous {
            self.get_previous_id()
        } else {
            self.get_next_id()
        }
    }
}

impl fmt::Display for WorkspaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current ID:\t{}\nMonitor IDs:\t{:?}\nOccupied IDs:\t{:?}", self.current_id, self.monitor_ids, self.occupied_ids)
    }
}
