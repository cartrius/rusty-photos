use aws_sdk_s3::Client as S3Client;
use std::error::Error;

pub async fn download_object(
    s3_client: &S3Client,
    bucket: &str,
    key: &str
) -> Result<Vec<u8>, Box<dyn Error>> {
    let response = s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    // Raw byte buffer for image processing/saving
    let data = response.body.collect().await?.into_bytes();
    Ok(data.to_vec())
}