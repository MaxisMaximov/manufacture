use std::{fs::File, io::BufReader};

use serde_json;

use super::*;

// TODO: maybe switch to `json-rust` for faster speed and lower memory usage
// Yes I'm concerned about these things early on
// No I don't have any idea why

pub fn init() {
    statics::SYS_debug.lock().unwrap().DEBUG_items.insert(
        ">SYS_SSINIT_json".to_string(),
        debug::DEBUG_item::new(
            debug::DEBUG_class::info,
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_json",
            MISC::PATHS::PATH_DEBUG,
            &[],
            40,
        ),
    );
}

/// # Fetch data from `.json` file
/// If it finds nothing it will return `Err()`
pub fn debugStr(IN_index: &str, IN_filePath: &str) -> Result<String, ()> {

    // Check of file even exists
    let idkfa_reader = BufReader::new(
        match File::open(IN_filePath){
            Ok(FILE) => FILE,
            Err(_) => 
            {
                statics::SYS_debug.lock().unwrap().DEBUG_items.insert(
                    "idkfa".to_string(),
                    debug::DEBUG_item::new(
                        debug::DEBUG_class::error,
                    ".ERR_json/!JSON_noFile",
                    vars::MISC::PATHS::PATH_DEBUG,
                    &[("{path}", IN_filePath.to_owned())],
                    40
                ));
                return Err(());
            },
        });
    
    // If all good retrieve data
    let mut W_retrievedData: serde_json::Value = serde_json::from_reader(idkfa_reader).unwrap();

    // Delve into the tree to find the string
    for NEXTINDEX in IN_index.split('/') {
        let idkfa_value = &W_retrievedData[NEXTINDEX];

        // If nothing is found, returns an error and sends it to Error Manager
        // I gotta find a cleaner way for this
        if idkfa_value.is_null() {

            // Just not to Deadlock
            if let Ok(mut DEBUG) = statics::SYS_debug.try_lock(){
                DEBUG.DEBUG_items.insert(
                    "iddqd".to_owned(),
                    debug::DEBUG_item::new(
                        debug::DEBUG_class::error,
                    ".JSON/!readString",
                    vars::MISC::PATHS::PATH_DEBUG,
                    &[("{id}", IN_index.to_owned()), ("{file}", IN_filePath.rsplit_once('/').unwrap().1.to_owned())],
                    40,
                ));
            }
            return Err(());
        }

        // Gotta do this for wahtever reason
        W_retrievedData = idkfa_value.clone()
    }

    // And return data
    return Ok(W_retrievedData.as_str().unwrap().to_string());
}