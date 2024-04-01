use std::{fs::File, io::BufReader};

use serde_json;

// TODO: maybe switch to `json-rust` for faster speed and lower memory usage
// Yes I'm concerned about these things early on
// No I don't have any idea why

pub struct SYS_jsonManager{
    // Commented out cuz I guess struct readers can't be used more than once
    //JSON_READER_debug: BufReader<File>
}
impl SYS_jsonManager {
    pub fn new() -> Self{
        SYS_jsonManager{
            // JSON_READER_debug: BufReader::new(File::open("./src/debug.json").unwrap())
        }
    }
    /// # Fetch debug string from `debug.json`
    /// If it finds nothing it will return the string ID instead
    pub fn JSON_FETCH_debugStr(&mut self, IN_index: &str) -> String{
        let idkfa_reader = BufReader::new(File::open("./src/debug.json").unwrap());
        let mut W_retrievedData: serde_json::Value = serde_json::from_reader(idkfa_reader).unwrap();

        // Delve into the tree to find the string
        for NEXTINDEX in IN_index.split("/"){
            let idkfa_value = &W_retrievedData[NEXTINDEX];

            // If nothing is found, just return the original index
            // I gotta find a cleaner way for this
            if idkfa_value == &serde_json::Value::Null{
                return IN_index.to_string();
            }
            W_retrievedData = idkfa_value.clone()
        }
        return W_retrievedData.as_str().unwrap().to_string();
    }
}