use std::{fs::File, io::BufReader};

use serde_json;

use crate::*;

// TODO: maybe switch to `json-rust` for faster speed and lower memory usage
// Yes I'm concerned about these things early on
// No I don't have any idea why

pub fn init() {
    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SSINIT_json".to_string(),
        IDDQD_textItem::new(
            renderer::RENDER_position::None,
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_json",
            "",
            40,
        ),
    );
}

/// # Fetch data from file
/// If it finds nothing it will return `Err()`
pub fn debugStr(IN_index: &str, IN_filePath: &str) -> Result<String, ()> {
    let idkfa_reader = BufReader::new(
        match File::open(IN_filePath){
            Ok(FILE) => FILE,
            Err(_) => 
            {
                SYS_errorQueue.lock().unwrap().push(SYS_ERROR::new(
                    ".ERR_json/!JSON_noFile",
                    MISC::PATHS::PATH_ERROR,
                    &vec![("{PATH}", IN_filePath.to_string())],
                    40,
                ));
                return Err(());
            },
        });
    let mut W_retrievedData: serde_json::Value = serde_json::from_reader(idkfa_reader).unwrap();

    // Delve into the tree to find the string
    for NEXTINDEX in IN_index.split('/') {
        let idkfa_value = &W_retrievedData[NEXTINDEX];

        // If nothing is found, returns an error and sends it to Error Manager
        // I gotta find a cleaner way for this
        if idkfa_value.is_null() {

            // Just not to Deadlock
            if IN_index == ".ERR_json/!JSON_readString"{
                return Err(())
            }
            SYS_errorQueue.lock().unwrap().push(SYS_ERROR::new(
                    ".ERR_json/!JSON_readString",
                    MISC::PATHS::PATH_ERROR,
                    &vec![("{ID}", IN_index.to_string()), ("{FILE}", IN_filePath.rsplit_once('/').unwrap().1.to_owned())],
                    40,
                ));
            return Err(());
        }
        W_retrievedData = idkfa_value.clone()
    }
    return Ok(W_retrievedData.as_str().unwrap().to_string());
}