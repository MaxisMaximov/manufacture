use super::*;

pub fn init() {
    statics::SYS_debug.lock().unwrap().DEBUG_items.insert(
        "#SSINIT_error".to_string(),
        debug::DEBUG_item::new(
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_error",
            MISC::PATHS::PATH_DEBUG,
            "",
            40,
        ),
    );
}

/// # Error Struct
/// Provide with ID, path of the file and collection of values to format with  
/// It will try to find the ID in any .json file you've specified  
///
/// ## DISCLAIMER
/// Remember to categorize the specs in your custom error file  
/// It'll spare the code spaghetti and you headaches
pub struct SYS_ERROR {
    pub ERR_spec: String,
    pub ERR_fullSpec: String,
    pub ERR_desc: String,
    pub ERR_lifetime: u16,
    pub ERR_markForDel: bool
}
impl SYS_ERROR {
    pub fn new(
        IN_spec: &str,
        IN_errorPath: &str,
        IN_values: &Vec<(&'static str, String)>,
        IN_lifetime: u16,
    ) -> Self {
        // Check if the error desc even exists
        let idkfa_fmtString = match json::debugStr(IN_spec, IN_errorPath) {
            Ok(mut DESCSTRING) => {
                // If exists, format it
                for VALUE in IN_values {
                    DESCSTRING = DESCSTRING.replace(VALUE.0, &VALUE.1);
                }
                DESCSTRING
            }
            Err(_) => "NO ERROR DESC FOUND, PLS FIX".to_owned(),
        };

        Self {
            ERR_spec: IN_spec.rsplit_once('/').unwrap().1.to_owned(),
            ERR_fullSpec: IN_spec.to_owned(),
            ERR_desc: idkfa_fmtString,
            ERR_lifetime: IN_lifetime,
            ERR_markForDel: false,
        }
    }
    pub fn ERR_tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.ERR_markForDel {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.ERR_lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.ERR_lifetime == 0 {
            self.ERR_markForDel = true;
            return;
        }
        self.ERR_lifetime -= 1;
    }
}

// Standard display
impl fmt::Display for SYS_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <> {}", self.ERR_spec, self.ERR_lifetime)
    }
}

// Specifics display
impl fmt::Debug for SYS_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} <> {}f",
            self.ERR_fullSpec, self.ERR_desc, self.ERR_lifetime
        )
    }
}

pub struct MASTER_errorQueue{
    pub inner: Vec<self::SYS_ERROR>
}
impl MASTER_errorQueue{
    pub fn new() -> Self{
        Self{
            inner: Vec::new()
        }
    }
    pub fn cleanup(&mut self){
        self.inner.retain(|v| !v.ERR_markForDel)
    }
}