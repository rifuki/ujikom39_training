use actix_web::{HttpResponse, Responder, web, http::header};
use serde_json::json;
use crate::{Pool, pembeli::{models::Pembeli, services}};
use tera::Tera;
use serde::Serialize;

#[derive(Serialize)]
struct BuyersContext {
    buyers: Vec<Pembeli>,
}

pub async fn get_buyers(pool: web::Data<Pool>) -> impl Responder {
    match services::get_buyers(&pool).await {
        Ok(all_buyers) => {
            let tera = Tera::new("templates/**/*").expect("Failed to initialize tera");
            let buyers_context = BuyersContext { buyers: all_buyers };
            let mut context = tera::Context::new();
            context.insert("buyers", &buyers_context);
            let rendered = tera.render("get_buyers.html", &context).expect("Failed to render template"); 
            HttpResponse::Ok().body(rendered)
        }
        Err(error) => {
            HttpResponse::InternalServerError().json(json!({ "msg": format!("{:?}", error)}))
        }
    }
}

pub async fn get_buyer() -> impl Responder {
    HttpResponse::Ok().body("get user")
}

pub async fn create_buyer(
    pool: web::Data<Pool>,
    payload: web::Json<Pembeli>,
) -> impl Responder {
    match services::create_buyer(&pool, payload.into_inner()).await {
        Ok(_) => {
            let redirect_url = "http://localhost/pembeli";  // Change this to your desired URL
            HttpResponse::Found()
                .append_header((header::LOCATION, redirect_url))
                .finish()
            },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn update_buyer() -> impl Responder {
    HttpResponse::Ok().body("Update User")
}

pub async fn delete_buyer(pool: web::Data<Pool>, id_pembeli: web::Path<i32>) -> impl Responder {
    match services::delete_buyer(&pool, id_pembeli.into_inner()).await {
        Ok(delete_pembeli) => HttpResponse::Ok().json(json!({
            "msg": format!("user {} successfully deleted", delete_pembeli.nama_pembeli)
        })),
        Err(error) => {
            HttpResponse::InternalServerError().json(json!({ "error": format!("{:?}", error) }))
        }
    }
}
