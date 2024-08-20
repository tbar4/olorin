use std::fs;

pub struct SchemaPuller{}

impl SchemaPuller {
    pub fn get_files() -> Vec<String> {
        let files = fs::read_dir("./schemas").expect("unable to read directory");
        
        let mut file_vec = vec![];
        
        for file in files {
            file_vec.push(file.unwrap().file_name().into_string().unwrap())
        }

        file_vec
    }
}