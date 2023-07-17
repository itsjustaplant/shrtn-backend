use actix_web::{HttpServer, App, http};
use actix_cors::Cors;

use crate::endpoints::link_endpoints::{get_link, create_link};

pub async fn start_server() -> std::io::Result<()> {
    println!("Starting Server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new().wrap(cors).service(get_link).service(create_link)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
