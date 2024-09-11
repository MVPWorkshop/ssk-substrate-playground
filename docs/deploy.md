# Project Deployment Guide

This guide log all the necessary steps to deploy rust project in to a production environment.

## Prerequisites

Ensure the following are installed on the server:
- Rust toolchain (Optional if deploying a pre-built binary)
- Docker (Optional for containerization)
- Systemd (Linux only)
- SSH access to the server

---

## 1. Build the Project

For production, the project should be built in release mode to optimize performance.

### Build in Release Mode

From your local machine, navigate to the project directory and run:

```bash
cargo build --release
```
This will create an optimized binary in the target/release/ directory.

## 2. Transfer the Binary to the Production Server
Once the project is built, transfer the release binary to the production server. You can use scp to securely copy files over SSH:

```bash
scp target/release/substrate-runtime-builder user@your-server:/path/to/deploy/
```

Replace my-project with your binary's name, user with the server's username, and /path/to/deploy/ with the desired destination path on the server.

### 3. Configure the Server

#### Install Necessary Libraries
If your project relies on external libraries (e.g., OpenSSL), ensure they are installed on the server. For example, on Debian/Ubuntu:

```bash
sudo apt update
sudo apt install libssl-dev -y
```

### 4. Run the Application
Navigate to the directory where you uploaded the binary and run it:

```bash
cd /path/to/deploy/
./substrate-runtime-builder
```

If you directly want to run the application after build you can use below command:

```
./target/release/substrate-runtime-builder 
```


### 5. Set Up as a Systemd Service (Optional)
If your Rust application needs to run continuously (like a web server or background service), you can create a systemd service to manage it.


### 6. Optional: Docker Containerization
A. Create a Dockerfile
If you prefer to deploy your Rust project using Docker, create a Dockerfile in the project root:

```bash
# Use an official Rust image as a base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Create a smaller image with just the binary
FROM debian:buster-slim
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/my-project /usr/local/bin/my-project

# Expose the necessary ports (if applicable)
EXPOSE 8080

# Run the binary
CMD ["my-project"]
```

B. Build and Run the Docker Image
Build the Docker image:

```bash
docker build -t my-rust-app .
```

Run the Docker container:

```bash
docker run -d -p 8080:8080 my-rust-app
```

### 7. Logs and Monitoring
#### A. Logs
If using systemd, logs are captured by journald. To view logs:

```bash
journalctl -u my-project -f
```

If you're using Docker, view logs with:

```bash
docker logs <container-id>
```

#### B. Monitoring
You can monitor your application’s performance using tools like Prometheus, Grafana, or Datadog. Ensure that your application exposes metrics if necessary.

### 8. Testing Production Deployment
Once your application is deployed, ensure that all functionality works as expected in the live environment:

* Check logs for errors.
* Run test transactions or requests to ensure the service responds as expected.
* Monitor system resource usage (CPU, memory).

### 9. Security Considerations

* Use HTTPS: If applicable, configure a reverse proxy (like NGINX) to handle SSL/TLS certificates using Let's Encrypt.
* Environment Variables: Store sensitive information (like API keys and database URLs) in environment variables, never in source code.

### 10. Future Updates and CI/CD
* To automate deployment for future updates:
* Set up a CI/CD pipeline (using GitHub Actions, GitLab CI, or Jenkins).
* Automate testing, building, and deployment to your production servers.