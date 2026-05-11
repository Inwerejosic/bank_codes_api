# Bank Codes API

A lightweight, high-performance REST API for serving Nigerian bank codes data, built with Rust and Axum.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org/)
[![Docker Hub](https://img.shields.io/badge/Docker-Hub-blue?logo=docker)](https://hub.docker.com/r/inwerejosic/bank-codes-api)

## Description

This project provides a simple HTTP API that serves static JSON data containing Nigerian bank codes, names, and categories. The data is extracted from PDF sources and served via RESTful endpoints with support for querying specific fields.

## Features

- 🚀 **High Performance**: Built with Rust and Axum for low latency and high throughput
- 📊 **JSON API**: RESTful endpoints for accessing bank data
- 🔍 **Query Support**: Dot-notation path queries for nested data access
- 🐳 **Docker Ready**: Containerized with optimized Alpine-based images (~25MB)
- 🔒 **Security**: Non-root user execution and minimal attack surface
- 📝 **Open Source**: MIT licensed for community contributions

## Quick Start

### Prerequisites

- Rust 1.75+ (for local development)
- Docker (for containerized deployment)

### Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/bank-codes-api.git
   cd bank-codes-api
   ```

2. **Install dependencies**
   ```bash
   cargo build --release
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

   The API will be available at `http://127.0.0.1:3000`

### Docker Compose Deployment

For easier deployment with Docker Compose:

```bash
# Build and run with docker-compose
docker-compose up --build

# Or run in background
docker-compose up -d --build
```

The service will be available at `http://localhost:3000`

## API Documentation

### Endpoints

#### GET `/`
Returns API health status.

**Response:**
```json
{
  "status": "ok"
}
```

#### GET `/api/data`
Returns all bank data.

**Response:**
```json
{
  "banks": [
    {
      "code": "000001",
      "name": "Sterling Bank",
      "category": 2
    }
  ]
}
```

#### POST `/api/query`
Query specific data using JSON request body.

**Request Body:**
```json
{
  "path": "banks.0.name"
}
```

**Response:**
```json
{
  "result": "Sterling Bank"
}
```

### Query Examples

```bash
# Get all data
curl http://127.0.0.1:3000/api/data

# Query first bank name
curl -X POST http://127.0.0.1:3000/api/query \
  -H "Content-Type: application/json" \
  -d '{"path": "banks.0.name"}'

# Query bank by code
curl -X POST http://127.0.0.1:3000/api/query \
  -H "Content-Type: application/json" \
  -d '{"path": "banks.0"}'
```

## Development

### Project Structure

```
bank-codes-api/
├── src/
│   └── main.rs              # Main application code
├── Cargo.toml               # Rust dependencies
├── Cargo.lock               # Dependency lock file
├── Dockerfile               # Docker build configuration
├── docker-compose.yml       # Docker Compose configuration
├── .dockerignore            # Docker build ignore rules
├── .gitignore               # Git ignore rules
├── data.json                # Bank codes data
└── README.md                # This file
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Docker Development

```bash
# Build for development
docker build -t bank-codes-api:dev .

# Run with volume mounting for live reload
docker run -p 3000:3000 -v $(pwd):/app bank-codes-api:dev

# Or use Docker Compose for development
docker-compose up --build
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Guidelines

- Follow Rust best practices and formatting (`cargo fmt`, `cargo clippy`)
- Add tests for new features
- Update documentation as needed
- Ensure Docker builds pass

## License

This project is licensed under the MIT License. Please see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Serde](https://serde.rs/) - Serialization framework
- [Data Source](https://globusbank.com/Files/List-of-Nigerian-Banks.pdf) 

## Support

If you find this project helpful, please consider:
- ⭐ Starring the repository
- 🐛 Reporting issues
- 💡 Suggesting features
- 🤝 Contributing code