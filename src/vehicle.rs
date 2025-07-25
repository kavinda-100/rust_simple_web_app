use axum::extract::{Query, Path};
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
// get the vehicle details
pub async fn vehicle_get() -> impl IntoResponse {
    let vehicle = Vehicle {
        id: Some(uuid::Uuid::new_v4().to_string()),
        manufacturer: "BMW".to_string(),
        name: "BMW".to_string(),
        model: "CS".to_string(),
        year: 2024,
    };
    (StatusCode::OK, Json(vehicle))
}

// get vehicle details by query parameter
pub async fn vehicle_get_by_query(Query(mut v): Query<Vehicle>) -> impl IntoResponse {
    // If the ID is not set, assign a new UUID
    if v.id.is_none() {
        v.id = Some(uuid::Uuid::new_v4().to_string());
    }
    // Log the received vehicle query
    println!("Received vehicle query: {:?}", v);
    
    // Here you would typically fetch the vehicle from a database or perform some action
    // For now, we just return the vehicle as is
    (StatusCode::OK, Json(v))
}

// post a new vehicle
pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> impl IntoResponse {
    // Assign a new UUID to the vehicle if it doesn't have one
    v.id = Some(uuid::Uuid::new_v4().to_string());
    // Log the received vehicle
    println!("Received vehicle: {:?}", v);
    // Here you would typically save the vehicle to a database or perform some action
    // For now, we just print it to the console
    (StatusCode::CREATED, Json(v)) // Return 201 Created with the vehicle data
}

// delete a vehicle
pub async fn vehicle_delete(Path(id): Path<String>) -> impl IntoResponse {
    // Log the received vehicle ID for deletion
    println!("Deleting vehicle with ID: {}", id);
    
    // Here you would typically delete the vehicle from a database
    // For now, we just return a success response
    (StatusCode::NO_CONTENT, format!("Vehicle with ID {} deleted successfully", id))
}