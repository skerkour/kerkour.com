use axum::{
    body::StreamBody,
    extract::{self, BodyStream},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{Stream, StreamExt};
use rusoto_core::ByteStream;
use rusoto_s3::S3;
use std::{
    io::{self},
    net::SocketAddr,
};
use tokio::sync::mpsc;
use tracing::info;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use tokio_util::io::StreamReader;
// use aws_sdk_s3::{Client, Config, Credentials, Endpoint, Region};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct GetFile {
//     path: String,
// }

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "rust-web=debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    std::env::set_var("RUST_LOG", "debug");

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().json())
        .init();

    info!("Hello world");

    // build our application with some routes
    let app = Router::new()
        .route("/multipart_stream_to_s3", post(multipart_stream_to_s3))
        .route("/stream_from_s3", get(stream_from_s3));
    // .layer(Extension(pool));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listenning on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn multipart_stream_to_s3(mut multipart: extract::Multipart) {
    let s3_client = rusoto_s3::S3Client::new(rusoto_core::Region::EuCentral1);

    // while let Some(mut field) = multipart
    //     .next_field()
    //     .await
    //     .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string())).unwrap()
    // {
    //     while let Some(chunk) = field
    //         .chunk()
    //         .await
    //         .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    //     {
    //         println!("received {} bytes", chunk.len());
    //     }
    // }

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        // let name = field.name().unwrap().to_string();
        // let file_name = field.file_name().unwrap().to_string();
        // let content_type = field.content_type().unwrap().to_string();

        let (tx, rx) = mpsc::channel(10);

        let bytes_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))
            .unwrap()
        {
            // println!("received {} bytes", chunk.len());
            tx.send(chunk).await.unwrap();
        }

        // let data = field.chunk().await.unwrap().unwrap();

        // Convert the stream into an `AsyncRead`.
        // let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        // let body_reader = StreamReader::new(body_with_io_error);
        // futures::pin_mut!(body_reader);

        // let mut byte_stream = BufWriter::new(ByteStream::from(Vec::new()));

        // tokio::io::copy(&mut body_reader, &mut byte_stream).await?;
        // println!(
        //     "Length of `{}` (`{}`: `{}`) is {} bytes",
        //     name,
        //     file_name,
        //     content_type,
        //     data.len()
        // );
        let stream: Box<dyn Stream<Item = Result<bytes::Bytes, io::Error>> + Send + Sync + Unpin> =
            Box::new(bytes_stream.map(Ok));
        let byte_stream = ByteStream::new(stream);

        // let byte_stream = ByteStream::new(stream::once(async { Ok(data) }));
        let s3_req = rusoto_s3::PutObjectRequest {
            bucket: "testaxums3".to_string(),
            key: "test.pdf".to_string(),
            body: Some(byte_stream),
            ..Default::default()
        };

        // tokio::io::copy(&mut body_reader, &mut byte_stream).await?;

        let _s3_res = s3_client.put_object(s3_req).await.unwrap();
    }
}

// async fn s3_upload_multipart(
//     mut multipart: extract::Multipart,
// ) -> Result<(), (StatusCode, String)> {
//     let s3_client = rusoto_s3::S3Client::new(rusoto_core::Region::EuCentral1);

//     while let Some(field) = multipart
//         .next_field()
//         .await
//         .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
//     {
//         let name = field.name().unwrap_or("name");

//         let stream: Box<dyn Stream<Item = Result<bytes::Bytes, io::Error>> + Send + Sync + Unpin> =
//             Box::new(
//                 field.map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))),
//             );

//         let byte_stream = ByteStream::new(stream);
//         let s3_req = rusoto_s3::PutObjectRequest {
//             bucket: "testaxums3".to_string(),
//             key: "test.pdf".to_string(),
//             body: Some(byte_stream),
//             ..Default::default()
//         };
//         let s3_res = s3_client.put_object(s3_req).await.unwrap();
//         // while let Some(chunk) = field
//         //     .chunk()
//         //     .await
//         //     .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
//         // {
//         //     println!("received {} bytes", chunk.len());
//         // }
//     }

//     Ok(())
// }

async fn body_stream_to_s3(mut stream: BodyStream) {
    let s3_client = rusoto_s3::S3Client::new(rusoto_core::Region::EuCentral1);

    let stream: Box<dyn Stream<Item = Result<bytes::Bytes, io::Error>> + Send + Sync + Unpin> =
        Box::new(
            stream.map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))),
        );

    let byte_stream = ByteStream::new(stream);
    let s3_req = rusoto_s3::PutObjectRequest {
        bucket: "testaxums3".to_string(),
        key: "test.pdf".to_string(),
        body: Some(byte_stream),
        ..Default::default()
    };
    let _s3_res = s3_client.put_object(s3_req).await.unwrap();

    // let config = Config::builder()
    //         .region(Region::new("us-east-1"))
    //         .build();
    // let s3_client = Client::from_conf(config);
    //         let stream: Box<dyn Stream<Item = Result<bytes::Bytes, io::Error>> + Send + Sync + Unpin> =
    //         Box::new(
    //             stream.map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))),
    //         );

    //     let byte_stream = aws_sdk_s3::types::ByteStream::from(stream);
    // s3_client
    //         .put_object()
    //         .key("key")
    //         .body(byte_stream)
    //         .bucket("bucket")
    //         .send()
    //         .await.unwrap();
}

async fn stream_from_s3() -> impl IntoResponse {
    // ) -> Result<StreamBody<ByteStream>, anyhow::Error> {
    let s3_client = rusoto_s3::S3Client::new(rusoto_core::Region::EuCentral1);

    let s3_req = rusoto_s3::GetObjectRequest {
        bucket: "testaxums3".to_string(),
        key: "test.pdf".to_string(),
        ..Default::default()
    };

    let s3_res = s3_client.get_object(s3_req).await.unwrap();

    StreamBody::new(s3_res.body.unwrap())
}

// Utility function for mapping any error into a `500 Internal Server Error`
// response.
// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
