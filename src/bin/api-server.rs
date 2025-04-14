use aws_sdk_s3::presigning::PresigningConfig;
use axum::{routing::{get, delete}, response::IntoResponse, Router, extract::{Path, Query}, Json};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::error::Error;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars
    dotenv::dotenv().ok();

    // Load AWS config
    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3_client = S3Client::new(&shared_config);

    let cors = CorsLayer::permissive();

    // Build Axum router
    let app = Router::new().route(
        "/get-upload-url",
        get({
            let s3_client = s3_client.clone();
            move |query: Query<GetUploadUrlParams>| {
                get_upload_url_handler(query, s3_client.clone())
            }
        }),
    ).route(
        "/list-images", get({
            let s3 = s3_client.clone();
            move || list_images_handler(s3.clone())
        })
    ).route(
        "/photos/*key", // Catch-all if your key has slashes
        delete({
            let s3 = s3_client.clone();
            move |path: Path<String>| delete_photo_handler(path, s3.clone())
        }),
    )
    .layer(cors);

    // Define the port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Rusty Photo API listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

// Define query params struct to parse ?key=<S3-object-key> from URL
#[derive(Deserialize)]
struct GetUploadUrlParams {
    key: String,
}

#[derive(Serialize)]
struct ImageListResponse {
    images: Vec<String>,
}

// GET /get-upload-url?key=somepath
async fn get_upload_url_handler(
    Query(params): Query<GetUploadUrlParams>,
    s3_client: S3Client
) -> String {
    // Read the bucket name from an environment variable
    // Default to "my-photo-bucket", if it's not set
    let bucket = std::env::var("UPLOAD_BUCKET").unwrap_or_else(|_| "my-photo-bucket".to_string());
    let request = s3_client
        .put_object()
        .bucket(&bucket)
        .key(&params.key);

    // Generate a presigned request, valid for a default expriation of 15 min
    let presigned_req = request
        .presigned(PresigningConfig::expires_in(Duration::from_secs(60 * 15))
        .expect("15 minutes")).await.unwrap();

    presigned_req.uri().to_string()
}

// GET /list-images
async fn list_images_handler(s3_client: S3Client) -> Json<ImageListResponse> {
    let bucket = std::env::var("S3_BUCKET_NAME").unwrap();

    let response = s3_client
        .list_objects_v2()
        .bucket(&bucket)
        .prefix("processed/")
        .send()
        .await
        .unwrap();

    let mut keys = Vec::new();
    for obj in response.contents() {
        if let Some(key) = obj.key() {

            // Helps eliminate 0 byte included-object within S3 folder
            if !key.ends_with(".jpg") && !key.ends_with(".png") && !key.ends_with(".jpeg") {
                continue;
            }
            
            let url = format!("https://{bucket}.s3.amazonaws.com/{key}");
            keys.push(url);
        }
    }

    Json(ImageListResponse { images: keys })
}

pub async fn delete_photo_handler(
    Path(key): Path<String>,
    s3_client: S3Client,
) -> impl IntoResponse {
    // Read the bucket from env/config
    let bucket = std::env::var("S3_BUCKET_NAME")
        .unwrap_or_else(|_| "my-photo-bucket".to_string());

    // Delete the original
    if let Err(e) = s3_client
        .delete_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
    {
        eprintln!("Error deleting original file {}: {:?}", key, e);
    }

    // Figure out processed variants (Ex: uploads/phto1.jpg may also be processed/phto1.jpg)
    let filename = key.rsplit('/').next().unwrap_or("file.jpg");
    let thumb_key = format!("processed/thumb_{}", filename);
    let medium_key = format!("processed/medium_{}", filename);

    // Delete each processed variant
    let _ = s3_client
        .delete_object()
        .bucket(&bucket)
        .key(thumb_key.clone())
        .send()
        .await;

    let _ = s3_client
        .delete_object()
        .bucket(&bucket)
        .key(medium_key.clone())
        .send()
        .await;

    println!("Deleted photo {} and processed variants", key);
}