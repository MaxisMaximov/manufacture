use std::{fs::File, io::BufReader};

use serde_json;

use crate::{IDDQD_textItem, SYS_debug, SYS_ERROR};

// TODO: maybe switch to `json-rust` for faster speed and lower memory usage
// Yes I'm concerned about these things early on
// No I don't have any idea why

pub fn init(){
    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SSINIT_json".to_string(), 
        IDDQD_textItem::newDebug(
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_json", 
            "", 
            40)
    );
}

/// # Fetch debug string from `debug.json`
/// If it finds nothing it will return `Err()`
pub fn debugStr(IN_index: &str) -> Result<String, SYS_ERROR> {
    let idkfa_reader = BufReader::new(File::open("./src/json/debug.json").unwrap());
    let mut W_retrievedData: serde_json::Value = serde_json::from_reader(idkfa_reader).unwrap();

    // Delve into the tree to find the string
    for NEXTINDEX in IN_index.split("/") {
        let idkfa_value = &W_retrievedData[NEXTINDEX];

        // If nothing is found, just return the original index
        // I gotta find a cleaner way for this
        if idkfa_value.is_null() {
            return Ok(NEXTINDEX.to_string());
        }
        W_retrievedData = idkfa_value.clone()
    }
    return Ok(W_retrievedData.as_str().unwrap().to_string());
}
