pub mod models;
pub mod repository;

use std::net::TcpListener;

use zero2prod::run;

use zero2prod::repository::mongodb_repo::MongoRepo;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db: MongoRepo = MongoRepo::init().await;
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    run(listener, db)?.await
}
