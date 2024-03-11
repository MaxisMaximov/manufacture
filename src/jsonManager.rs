use std::{fs::File, io::{BufReader, Read}};

use serde_json;

pub struct SYS_jsonManager{
    JSON_READER_debug: BufReader<File>
}
impl SYS_jsonManager {
    pub fn new() -> Self{
        SYS_jsonManager{
            JSON_READER_debug: BufReader::new(File::open("./src/debug.json").unwrap())
        }
    }
    pub fn JSON_FETCH_debugStr(&mut self, IN_type: &str, IN_index: &str) -> String{
        let WORK_retrievedData: serde_json::Value = serde_json::from_reader(self.JSON_READER_debug.by_ref()).unwrap();
        return WORK_retrievedData[IN_type][IN_index].to_string();
    }
}