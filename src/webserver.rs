use std::{
    fs::File,
    io::{BufWriter, Write},
};

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Router,
};
use log::info;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn webserver() {
    let ip = "127.0.0.1:8080";

    let app = Router::new()
        .route("/", post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(ip).await.unwrap();

    info!("Web server started at {}", ip);
    axum::serve(listener, app).await.unwrap();
}

async fn accept_form(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        //let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        info!(
            "`{file_name}`: Content-Type: `{content_type}`, {} bytes",
            data.len()
        );

        write_to_file(data, file_name).await;
    }
}

async fn write_to_file(content: Bytes, file_name: String) {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join("Downloads").join(file_name);

    let f = File::create(&path).unwrap();
    let mut writer = BufWriter::new(f);
    writer.write_all(&content.to_vec()).unwrap();
    writer.flush().unwrap();

    info!("Saved at `{}`", path.to_str().unwrap());
}
