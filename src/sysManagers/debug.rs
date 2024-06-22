use super::*;

/// # Master Debug Struct
/// Holds the Debug info from subsystems
///
/// Reason I made this?  
/// So that Deadlocks don't happen with `SYS_data` because apparently it really likes to do that
pub struct DEBUG_master {
    pub DEBUG_items: HashMap<String, IDDQD_textItem>,
}
impl DEBUG_master {
    pub fn new() -> Self {
        Self {
            DEBUG_items: HashMap::new(),
        }
    }

    /// Clean up the hashmap
    /// A.k.a. get rid of `#MARK_FOR_DELETION` entries
    pub fn cleanup(&mut self) {
        self.DEBUG_items
            .retain(|_, v| !v.t_markForDel)
    }
}