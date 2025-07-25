use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    id: Option<String>,
    manufacturer: String,
    name: String,
    model: String,
    year: u32,
}
pub async fn vehicle_get() -> Json<Vehicle> {
    let vehicle = Vehicle {
        id: Some(uuid::Uuid::new_v4().to_string()),
        manufacturer: "BMW".to_string(),
        name: "BMW".to_string(),
        model: "CS".to_string(),
        year: 2024,
    };
    Json(vehicle)
}
pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> impl IntoResponse {
    // Assign a new UUID to the vehicle if it doesn't have one
    v.id = Some(uuid::Uuid::new_v4().to_string());
    // Log the received vehicle
    println!("Received vehicle: {:?}", v);
    // Here you would typically save the vehicle to a database or perform some action
    // For now, we just print it to the console
    (StatusCode::CREATED, Json(v)) // Return 201 Created with the vehicle data
}