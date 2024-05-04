use std::error::Error;
use postman_collection;
use postman_collection::PostmanCollection;

fn import(path: &str) -> Result<PostmanCollection, String> {
    let collection = postman_collection::from_path(path);

    return match collection {
        Ok(collection) => {
            Ok(collection)
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err("Failed to import collection".to_string())
        }
    }
}