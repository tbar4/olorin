use file_reader::SchemaPuller;
use olorin::extractors::*;
use serde_json::{from_reader, Value};


#[tokio::main]
async fn main() {
    let files = SchemaPuller::get_files();
    
    for f in files {
        let path = format!("./schemas/{f}");
        
        let input =json_api::JSONInput::build(path.as_str())
            .await
            .fetch()
            .await;
        
        println!("{:#?}", input);
    }
}
