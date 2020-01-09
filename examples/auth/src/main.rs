use actix_web::{HttpServer, web, HttpResponse, App};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};
use actix_auth::login_required;

#[login_required]
pub async fn index(_id: Identity) -> HttpResponse {
    HttpResponse::Ok().json(ident)
}

pub async fn login(_id: Identity) -> HttpResponse {
    _id.remember(String::from("test"));
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
pub async fn main() -> std::io::Result<()>{
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new("b736a79703ba26141ffb5ee0ed23c8ii".as_bytes())
                    .name("auth")
                    .path("/")
                    .max_age_time(chrono::Duration::hours(6))
                    .secure(false)
            ))
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/register")
                            .route(web::get().to(index))
                    )
                    .service(
                        web::resource("/login")
                            .route(web::get().to(login))
                    )
            )
    })
        .bind("127.0.0.1:1089")?
        .workers(4)
        .run()
        .await
}
