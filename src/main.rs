use std::error::Error;
use serde_json::Value;
mod s3_ops;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars
    dotenv::dotenv().ok();

    println!("Hello Rusty Photos!");
    Ok(())
}

fn parse_s3_event(msg_body: &str) -> Option<(String, String)> {
    let v: Value = serde_json::from_str(msg_body).ok()?;
    let records = v["Records"].as_array()?;
    if records.is_empty() {
        return None;
    }

    let bucket = records[0]["s3"]["bucket"]["name"].as_str()?.to_string();
    let key = records[0]["s3"]["object"]["key"].as_str()?.to_string();

    Some((bucket, key))
}
