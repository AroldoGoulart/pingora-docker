# Pingora-docker
## Description

This project is a simple implementation of Pingora, a Rust framework designed to build fast, reliable, and programmable networked systems. 
Pingora is battle-tested, having served more than 40 million internet requests per second for several years. It offers several features, including async Rust support, HTTP 1/2 end-to-end proxy, TLS over OpenSSL or BoringSSL, gRPC and WebSocket proxying, graceful reload, customizable load balancing and failover strategies, and support for various observability tools.

## Usage

### With Docker

To run this project using Docker, follow these steps:

1. Clone the repository.
2. Navigate to the project directory.
3. Build the Docker image using the provided Dockerfile:
   ```
   docker compose build
   ```
4. Run the Docker container:
   ```
   docker-compose up
   ```

### Without Docker

To run this project without Docker, follow these steps:

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project using Cargo:
   ```
   cargo build --release
   ```
4. Run the project:
   ```
   cargo run
   ```

## Additional Information

For more details about Pingora and its features, please refer to the [official Pingora repository](https://github.com/cloudflare/pingora).

## License

This project is licensed under the Apache License, Version 2.0.

---

**Note:** This project is a simple demonstration of Pingora's capabilities and does not encompass the full range of features offered by the framework.

---

### GitHub Description

Pingora is a Rust framework for building fast, reliable, and customizable networked systems. This project offers a simplified implementation of Pingora, showcasing its key features such as async Rust support, HTTP proxying, TLS, load balancing, and more. Ideal for those interested in exploring Pingora's capabilities in a concise example.