pub mod models;
pub mod repository;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web,
    web::{Data, Json, Path},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use repository::mongodb_repo::MongoRepo;

pub fn run(listener: TcpListener, db: MongoRepo) -> Result<Server, std::io::Error> {
    let db_data = Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/health", web::get().to(health))
            .route("/users/{id}", web::get().to(get_user))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_user(db: Data<MongoRepo>, req: HttpRequest) -> HttpResponse {
    let user_id = req.match_info().get("id").unwrap_or("0").to_string();

    if user_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&user_id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
