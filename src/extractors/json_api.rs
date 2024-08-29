use serde::Deserialize;
use serde_json::{Value, from_reader};

use super::Input;

#[derive(Debug, Deserialize)]
pub struct JSONInput {
    pub name: String,
    pub r#type: String,
    pub url: String,
    pub results_field_name: String,
    pub schema: Vec<Value> 
}

impl JSONInput {
    pub async fn build(file: &str) -> Self {
        let json = std::fs::File::open(file).expect("File not found!");
        let json_reader: JSONInput = from_reader(json).expect("Failed to read file into JSON, check build function");

        json_reader
    }

    pub async fn fetch(mut self) -> Self {
        let response = reqwest::get(&self.url)
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let results = &response[&self.results_field_name];

        self.schema = results.clone().as_array().unwrap().to_owned();
        
        self
    }
    
    pub async fn list_results(self) {
        println!("Name: {}", self.name);
        for (i, article) in self.schema.into_iter().enumerate() {
            println!("Article {i}:\n\t {:#?}", article);
        }
    }
    
    pub async fn to_input(self) -> Input {
        Input {
            name: self.name,
            r#type: self.r#type,
            url: self.url,
            results_field_name: self.results_field_name,
            schema: self.schema
        }
    }
}
