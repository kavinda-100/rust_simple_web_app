use axum::{routing::get, Router};
use axum::routing::post;
mod vehicle;
use vehicle::*;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .route("/vehicle", get(vehicle_get))
        .route("/vehicle", post(vehicle_post));

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    let app_res= axum::serve(listener, app).await;
    match app_res {
        Ok(_) => println!("Server running on http://localhost:5000"),
        Err(e) => eprintln!("Server error: {}", e),
    }
}

async fn hello() -> &'static str {
    "Hello, World from Axum With Rust! 🦀"
}


