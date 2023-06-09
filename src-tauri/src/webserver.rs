/*use axum::{
    extract::{ContentDisposition, Multipart},
    handler::post,
    Router,
};
use hyper::body::Bytes;
use sanitize_filename;
use std::convert::Infallible;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn upload(mut parts: Multipart) -> Result<&'static str, Infallible> {
    while let Ok(Some(mut field)) = parts.try_next().await {
        let content_disposition: ContentDisposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("./{}", sanitize_filename::sanitize(&filename));

        let mut file = File::create(filepath).await.unwrap();
        while let Some(result) = field.try_data().await {
            match result {
                Ok(data) => {
                    let bytes: Bytes = data;
                    file.write_all(&bytes).await.unwrap();
                }
                Err(_) => return Ok("Error while reading file data!"),
            }
        }
    }

    Ok("File uploaded successfully")
}

pub async fn run_server() {
    let app = Router::new().route("/upload", post(upload));
    let addr = "127.0.0.1:8000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
*/
