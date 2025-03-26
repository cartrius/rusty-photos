use aws_sdk_sqs::Client as SqsClient;
use aws_sdk_s3::Client as S3Client;
use std::error::Error;

pub async fn receive_messages(
    sqs_client: &SqsClient,
    queue_url: &str
) -> Result<Vec<Message>, Box<dyn Error>> {
    let response = sqs_client
        .receive_message()
        .queue_url(queue_url)
        .max_number_of_messages(10)
        .wait_time_seconds(20)
        .send()
        .await?;

    Ok(response.messages().unwrap_or_default().to_vec())
}

pub async fn delete_message(
    sqs_client: &SqsClient,
    queue_url: &str,
    receipt_handle: &str
) -> Result<(), Box<dyn Error>> {
    sqs_client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .send()
        .await?;

    Ok(())
}