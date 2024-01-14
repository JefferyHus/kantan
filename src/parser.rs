use serde_json::{Value, Error};

pub fn parse_json(json: &str) -> Result<Value, Error> {
    match serde_json::from_str(json) {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Error parsing JSON: {:?}", e);
            Err(e)
        }
    }
}