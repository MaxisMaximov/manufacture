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

/// # Error Struct
/// Provide with ID, path of the file and collection of values to format with  
/// It will try to find the ID in `error.json`  
/// 
/// Do not touch `.ERROR_json/!JSON_readString` or it will peck up
#[derive(Default)]
pub struct SYS_ERROR {
    pub ERR_spec: String,
    pub ERR_desc: String,
    pub ERR_lifetime: u16,
    pub ERR_markForDel: bool
}
impl SYS_ERROR {
    pub fn new(IN_spec: &str, IN_errorPath: &str, IN_values: &Vec<(&'static str, String)>, IN_lifetime: u16) -> Self{

        // Check if the error desc even exists
        let idkfa_fmtString = match jsonManager::debugStr(IN_spec, IN_errorPath){
            Ok(mut DESCSTRING) => {
                for VALUE in IN_values{
                    DESCSTRING = DESCSTRING.replace(VALUE.0, &VALUE.1);
                }
                DESCSTRING
            },
            Err(_) => "NO ERROR DESC FOUND, PLS FIX".to_owned(),
        };

        Self{
            ERR_spec: IN_spec.rsplit_once('/').unwrap().1.to_owned(),
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
            self.ERR_markForDel();
            return;
        }
        self.ERR_lifetime -= 1;
    }
    pub fn ERR_markForDel(&mut self) {
        self.ERR_markForDel = true;
        self.ERR_spec = "#MARK_FOR_DEL".to_owned()
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
            "{} | {} <> {}f",
            self.ERR_spec, self.ERR_desc, self.ERR_lifetime
        )
    }
}