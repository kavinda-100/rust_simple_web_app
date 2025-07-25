use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    id: String,
    manufacturer: String,
    name: String,
    model: String,
    year: u32,
}
pub async fn vehicle_get() -> Json<Vehicle> {
    let vehicle = Vehicle {
        id: uuid::Uuid::new_v4().to_string(),
        manufacturer: "BMW".to_string(),
        name: "BMW".to_string(),
        model: "CS".to_string(),
        year: 2024,
    };
    Json(vehicle)
}
pub async fn vehicle_post() {}