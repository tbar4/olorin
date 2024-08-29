use file_reader::SchemaPuller;
use olorin::{extractors::*, loaders::*};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let files = SchemaPuller::get_files();
    
    for f in files {
        let path = format!("./schemas/{f}");
        
        let input =json_api::JSONInput::build(path.as_str())
            .await
            .fetch()
            .await
            .to_input()
            .await;
    
        file_loader::Loader::new(input)
            .await
            .write_to_file(f)
            .await;
    }
    
    Ok(())
}
