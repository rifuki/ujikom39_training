use actix_cors::Cors;
use actix_web::{HttpServer, App, web, middleware::NormalizePath, HttpResponse};
use serde_json::json;
use ujikom39::{establish_connection, pembeli};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE URL must be set. [{}]", err);
        std::process::exit(1);
    });
    
    let pool = establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .configure(pembeli::routes::configure)
            .default_service(web::route().to(|| async { HttpResponse::NotFound().json(json!({"msg": "404 NOT FOUND"}))}))
    })
    .bind(("::", 80))?
    .run()
    .await
}
