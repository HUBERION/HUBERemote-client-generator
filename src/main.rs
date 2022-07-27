#![forbid(unsafe_code)]

use std::net::SocketAddr;

use axum::{Router, extract::Path, routing::get};
use axum::body::StreamBody;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use tokio_util::io::ReaderStream;

type AppError = (StatusCode, &'static str);

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:id", get(download));

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));

    println!("Server listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e));
}

async fn download(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let file = tokio::fs::File::open("bin/supporter.exe")
        .await.map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "Could not open supporter.exe"))?;

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.microsoft.portable-executable".to_string()),
        (header::CONTENT_DISPOSITION, format!(r#"attachment; filename="supporter-{id}.exe""#)),
    ];

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok((headers, body))
}