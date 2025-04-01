# Rusty Photo Manager

An event-driven image processing pipeline written in Rust, using AWS S3 and SQS.

Whenever a new image is uploaded to a configured S3 bucket (uploads/ prefix), the app:

Receives an event via SQS

Downloads the image from S3

Generates resized versions (e.g., thumbnail and medium)

Uploads them to a processed/ folder in the same bucket

