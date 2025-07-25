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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct VehicleQuery {
    id: Option<String>,
    manufacturer: Option<String>,
    name: Option<String>,
    model: Option<String>,
    year: Option<u32>,
}

// get all vehicles
pub async fn vehicle_get_all() -> impl IntoResponse {
    // Log the request for all vehicles
    println!("Received request for all vehicles");
    let vehicle_names = vec!["BMW", "Audi", "Mercedes", "Toyota", "Honda"];
    let vehicle_models = vec!["CS", "A4", "C-Class", "Camry", "Civic"];
    let mut vehicles: Vec<Vehicle> = Vec::new();
    for (i, name) in vehicle_names.iter().enumerate() {
        vehicles.push(Vehicle {
            id: Some(uuid::Uuid::new_v4().to_string()),
            manufacturer: name.to_string(),
            name: name.to_string(),
            model: vehicle_models[i].to_string(),
            year: 2024,
        });
    }
    // Return the list of vehicles as JSON
    (StatusCode::OK, Json(vehicles))
}

// get the vehicle details
pub async fn vehicle_get(Path(id): Path<String>) -> impl IntoResponse {
    // Log the received vehicle ID
    println!("Received vehicle ID: {}", id);
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
pub async fn vehicle_get_by_query(Query(v): Query<VehicleQuery>) -> impl IntoResponse {
    // If the ID is not set, assign a new UUID
    let query_vehicle = Vehicle {
        id: v.id.clone().or_else(|| Some(uuid::Uuid::new_v4().to_string())),
        manufacturer: v.manufacturer.clone().unwrap_or("Unknown".to_string()),
        name: v.name.clone().unwrap_or("Unknown".to_string()),
        model: v.model.clone().unwrap_or("Unknown".to_string()),
        year: v.year.unwrap_or(2024),
    };
    // Log the received vehicle query
    println!("Received vehicle query: {:?}", v);

    // Here you would typically fetch the vehicle from a database or perform some action
    // For now, we just return the vehicle as is
    (StatusCode::OK, Json(query_vehicle))
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

// put a vehicle (update)
pub async fn vehicle_put(Path(id): Path<String>, Json(mut v): Json<Vehicle>) -> impl IntoResponse {
    // Log the received vehicle for update
    println!("Updating vehicle: {:?}", v);
    // Log the vehicle ID being updated
    println!("Vehicle ID to update: {}", id);
    // Here you would typically update the vehicle in a database
    // For now, we just return the updated vehicle
    if v.id.is_none() {
        v.id = Some(uuid::Uuid::new_v4().to_string());
    }
    (StatusCode::OK, Json(v)) // Return 200 OK with the updated vehicle
}

// delete a vehicle
pub async fn vehicle_delete(Path(id): Path<String>) -> impl IntoResponse {
    // Log the received vehicle ID for deletion
    println!("Deleting vehicle with ID: {}", id);

    // Here you would typically delete the vehicle from a database
    // For now, we just return a success response
    (StatusCode::NO_CONTENT, Json(format!("Vehicle with ID {} deleted successfully", id)))
}