# 🚗 Rust Simple Web App - Vehicle REST API

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/axum-ff6600?style=for-the-badge&logo=rust&logoColor=white)
![REST API](https://img.shields.io/badge/REST-API-blue?style=for-the-badge)
![Learning](https://img.shields.io/badge/Purpose-Learning-green?style=for-the-badge)

A simple REST API built with **Rust** and the **Axum** web framework for learning purposes. This project demonstrates basic CRUD operations for vehicle management without database integration.

## 🎯 Project Goals

- 📚 Learn Rust programming language fundamentals
- 🌐 Understand REST API development with Axum
- 🔧 Practice CRUD operations (Create, Read, Update, Delete)
- 🚀 Build a foundation for future Rust web development

## ✨ Features

- ✅ **GET** - Retrieve all vehicles or specific vehicle by ID
- ✅ **POST** - Create new vehicles with auto-generated UUIDs
- ✅ **PUT** - Update existing vehicles
- ✅ **DELETE** - Remove vehicles by ID
- ✅ **Query Parameters** - Search vehicles with custom filters
- ✅ **JSON Response** - All endpoints return JSON data
- ✅ **HTTP Status Codes** - Proper status codes (200, 201, 204, etc.)

## 🛠️ Tech Stack

- **Language**: Rust 🦀
- **Web Framework**: Axum
- **Serialization**: Serde (JSON)
- **Async Runtime**: Tokio
- **UUID Generation**: uuid crate
- **HTTP Client Testing**: REST Client files

## 📊 API Endpoints

| Method | Endpoint | Description | Status Code |
|--------|----------|-------------|-------------|
| `GET` | `/` | Welcome message | 200 |
| `GET` | `/vehicle/all` | Get all vehicles | 200 |
| `GET` | `/vehicle/{id}` | Get vehicle by ID | 200 |
| `GET` | `/vehicle/query?params` | Search vehicles | 200 |
| `POST` | `/vehicle` | Create new vehicle | 201 |
| `PUT` | `/vehicle/{id}` | Update vehicle | 200 |
| `DELETE` | `/vehicle/{id}` | Delete vehicle | 204 |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd rust_simple_web_app
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

4. **Server will start on**
   ```
   🌐 http://localhost:5000
   ```

## 📝 Usage Examples

### Using the HTTP File

The project includes a `AxumWithRust.http` file for easy API testing. Open it in VS Code with the REST Client extension or any compatible IDE.

### Sample Requests

#### Create a Vehicle
```http
POST http://localhost:5000/vehicle
Content-Type: application/json

{
  "manufacturer": "Toyota",
  "name": "Camry",
  "model": "Camry",
  "year": 2023
}
```

#### Get All Vehicles
```http
GET http://localhost:5000/vehicle/all
```

#### Search Vehicles
```http
GET http://localhost:5000/vehicle/query?manufacturer=Toyota&year=2023
```

#### Update Vehicle
```http
PUT http://localhost:5000/vehicle/1
Content-Type: application/json

{
  "manufacturer": "Honda",
  "name": "Civic",
  "model": "Civic",
  "year": 2024
}
```

#### Delete Vehicle
```http
DELETE http://localhost:5000/vehicle/1
```

## 📁 Project Structure

```
rust_simple_web_app/
├── 📄 Cargo.toml          # Project dependencies
├── 📄 README.md           # Project documentation
├── 🧪 AxumWithRust.http   # HTTP test requests
├── 📂 src/
│   ├── 📄 main.rs         # Application entry point & routes
│   └── 📄 vehicle.rs      # Vehicle handlers & data structures
└── 📂 target/             # Compiled artifacts
```

## 🔧 Dependencies

```toml
[dependencies]
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
uuid = { version = "1.0", features = ["v4"] }
```

## 📚 Learning Objectives Covered

- ✅ **Rust Syntax**: Structs, enums, pattern matching, ownership
- ✅ **Async Programming**: Understanding `async/await` with Tokio
- ✅ **Web Development**: HTTP methods, status codes, JSON handling
- ✅ **Error Handling**: Result types and proper error responses
- ✅ **Code Organization**: Module system and separation of concerns
- ✅ **Testing**: HTTP file-based API testing

## 📄 License

This project is available for learning porous.

## Author
- [Kavinda Rathnayake](https://github.com/kavinda-100)

## 🙏 Acknowledgments

- 🦀 [Rust Community](https://www.rust-lang.org/)
- ⚡ [Axum Framework](https://github.com/tokio-rs/axum)
- 📚 [Rust Book](https://doc.rust-lang.org/book/)
- 🌐 [Tokio Async Runtime](https://tokio.rs/)

---

**Happy Learning with Rust! 🦀✨**

> *"The best way to learn is by building. Start simple, iterate, and grow."*
