use crate::*;

pub fn init(){
    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SSINIT_error".to_string(), 
        IDDQD_textItem::new(
            renderer::RENDER_position::None,
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_error", 
            "", 
            40)
    );
}

/// # Error Type
#[derive(PartialEq)]
pub enum SYS_ERRORTYPE {
    /// Error reading string from .json  
    /// (stringID, fileName)
    ERR_jsonRead(String, String),
    /// Error rendering border
    ERR_borderRender(vector2),
    /// Error rendering textBoxes
    ERR_textRender(vector2),
    /// Any error that doesn't match others
    ERR_misc(String),

    /// Mark for deletion
    ERR_markForDel,

    /// placeholder
    ERR_idkfa,
}
impl fmt::Display for SYS_ERRORTYPE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_string = match self {
            Self::ERR_jsonRead(ID, FILE) => {
                format!("ERR_jsonRead | Could not read string {ID} from {FILE}")
            }
            Self::ERR_borderRender(COORDS) => {
                format!("ERR_borderRender | Could not render border at {COORDS:?}")
            }
            Self::ERR_textRender(COORDS) => {
                format!("ERR_textRender | Text renderer outside of buffer at {COORDS:?}")
            }
            Self::ERR_misc(ERROR) => ERROR.to_string(),
            // Ignore those that are `ERR_idkfa` or `ERR_markForDel`
            _ => "".to_string()
        };
        write!(f, "{}", idkfa_string)
    }
}

/// # Error Struct
/// Provide with Error ID from `SYS_ERRORTYPE` and how long you want it to stay
pub struct SYS_ERROR {
    pub ERR_spec: self::SYS_ERRORTYPE,
    pub ERR_lifetime: u16,
}
impl SYS_ERROR {
    pub fn ERR_tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.ERR_spec == SYS_ERRORTYPE::ERR_markForDel {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.ERR_lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.ERR_lifetime == 0 {
            self.ERR_markForDel();
            return;
        }
        self.ERR_lifetime -= 1;
    }
    pub fn ERR_markForDel(&mut self) {
        *self = SYS_ERROR{
            ERR_spec: self::SYS_ERRORTYPE::ERR_markForDel,
            ERR_lifetime: 0}
    }
}
impl fmt::Display for SYS_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <> {}",
            self.ERR_spec, self.ERR_lifetime
        )
    }
}
impl fmt::Debug for SYS_ERROR{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <> {}",
            self.ERR_spec, self.ERR_lifetime
        )
    }
}
