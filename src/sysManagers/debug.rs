use super::*;

/// # Master Debug Struct
/// Holds the Debug info from subsystems
///
/// Reason I made this?  
/// So that Deadlocks don't happen with `SYS_data` because apparently it really likes to do that
pub struct DEBUG_master {
    pub DEBUG_items: HashMap<String, DEBUG_item>,
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
            .retain(|_, v| !v.markForDel)
    }
}

pub struct DEBUG_item {
    pub class: DEBUG_class,
    pub string: String,
    pub values: Vec<(&'static str, String)>,
    pub lifetime: u16,
    pub markForDel: bool,
}
impl DEBUG_item {
    /// # Create new TextItem
    ///
    /// The one used to place text somewhere in the game
    pub fn new(
        IN_class: DEBUG_class,
        IN_spec: &str,
        IN_debugPath: &str,
        IN_values: &[(&'static str, String)],
        IN_lifetime: u16,
    ) -> Self {
        // Check if it's a debug string
        
        Self {
            class: IN_class,
            string: json::debugStr(IN_spec, IN_debugPath).unwrap_or(IN_spec.to_string()),
            values: IN_values.to_vec(),
            lifetime: IN_lifetime,
            markForDel: false,
        }
    }

    /// Tickdown lifetime  
    /// Just to make it clean
    pub fn DEBUG_tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.markForDel {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.lifetime == 0 {
            self.markForDel = true;
            return;
        }
        self.lifetime -= 1;
    }
}

pub enum DEBUG_class{
    idkfa,
    info,
    warn,
    error,
    fatal
}

impl fmt:: Display for DEBUG_class{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_fmtString = match self{
            Self::idkfa => "*idkfa".with(Color::White),
            Self::info => "#info".with(Color::Yellow),
            Self::warn => "$warn".with(Color::Rgb {r: 255, g: 153, b: 0}),
            Self::error => "!error".with(Color::Magenta),
            Self::fatal => "%FATAL".with(Color::Red),
        };

        f.write_str(&idkfa_fmtString.to_string())
    }
}