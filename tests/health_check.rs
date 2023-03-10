use std::net::TcpListener;

use zero2prod::repository::mongodb_repo::MongoRepo;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let db = MongoRepo::init().await;
    let address = spawn_app(db);

    //Act
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

fn spawn_app(db: MongoRepo) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener, db).expect("Failed to spawn our app");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
