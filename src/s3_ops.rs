use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
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

pub async fn upload_object(
    s3_client: &S3Client,
    bucket: &str,
    key: &str,
    bytes: Vec<u8>,
    content_type: &str
) -> Result<(), Box<dyn Error>> {
    let body = ByteStream::from(bytes);

    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .content_type(content_type)
        .send()
        .await?;

    Ok(())

}