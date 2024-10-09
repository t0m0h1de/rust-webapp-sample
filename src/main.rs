use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
    });
    println!("Serving on http://localhost:8000...");
    server
        .bind("127.0.0.1:8000").expect("error binding server to address")
        .run().await.expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <html>
                <head>
                <meta charset="utf-8">
                <title>Sample Page</title>
                </head>
                <body>
                Sample Web Page
                </body>
                </html>
            "#,
        )
}
