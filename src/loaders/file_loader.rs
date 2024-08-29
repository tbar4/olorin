use std::{fs::File, io::{BufWriter, Write}};

use serde::{Deserialize, Serialize};
use crate::extractors::Input;

#[derive(Debug, Deserialize, Serialize)]
pub struct Loader {
    input: Input
}

impl Loader {
    pub async fn new(input: Input) -> Loader {
        Loader {
            input
        }
    }
    
    pub async fn write_to_file(self, file_name: String) {
        let file_path = format!("./{file_name}");
        let file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(file);
        
        serde_json::to_writer(&mut writer, &self.input.schema).unwrap();
        writer.flush().unwrap();
    }
}