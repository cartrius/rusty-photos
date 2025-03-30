use std::error::Error;
use std::time::Duration;
use serde_json::Value;

use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client as SqsClient;
use aws_sdk_s3::Client as S3Client;
use tokio::time::sleep;


mod s3_ops;
mod image_processing;
mod sqs_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars
    dotenv::dotenv().ok();

    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3_client = S3Client::new(&shared_config);
    let sqs_client = SqsClient::new(&shared_config);

    // Get the SQS queue URL from env or hardcode
    let queue_url = std::env::var("SQS_QUEUE_URL")
        .expect("Missing SQS_QUEUE_URL env var — did you load .env correctly?");

    println!("Rusty Photo Manager Started! Watching queue...");

    // SQS Loop
    loop {
        println!("Using SQS queue URL: {}", queue_url);
        let messages = sqs_handler::receive_messages(&sqs_client, &queue_url).await?;

        for msg in messages {
            if let Some(body) = msg.body() {
                if let Some((bucket, key)) = parse_s3_event(&body) {
                    // Skips over empty keys and folders (Ex: "upload/" events)
                    if key.ends_with('/') || key.is_empty() {
                        continue;
                    }
                    println!("New image uploaded: bucket={} key={}", bucket, key);

                    // Download image
                    let object_data = s3_ops::download_object(&s3_client, &bucket, &key).await?;
                    
                    // Process image
                    let processed_images = image_processing::process_image(object_data)?;
                    println!("Processed {} images", processed_images.len());

                    for (suffix, image_bytes) in processed_images {
                        // Build a new key (Ex: "processed/thumb_filename.jpg")
                        let filename = key.rsplit('/').next().unwrap_or("file.jpg");
                        let new_key = format!("processed/{}_{}", suffix, filename);

                        s3_ops::upload_object(&s3_client, &bucket, &new_key, image_bytes, "image/jpeg").await?;
                        println!("Uploaded {}", new_key);
                    }
                }
            }

            // Delete message after processing
            if let Some(receipt_handle) = msg.receipt_handle() {
                sqs_handler::delete_message(&sqs_client, &queue_url, receipt_handle).await?;
            }
        }
        
        // Wait before polling again
        sleep(Duration::from_secs(5)).await;
    }

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
