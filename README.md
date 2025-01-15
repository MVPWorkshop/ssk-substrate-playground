# Substrate Runtime Builder

This project allows users to generate Substrate projects with selected pallets and download them as zip files or publish them to your github. The project uses the `substrate-runtime-builder` crate to dynamically generate Substrate-based blockchain projects.

## Prerequisites

Before running this project, ensure that you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Substrate](https://github.com/paritytech/polkadot-sdk-solochain-template)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/MVPWorkshop/ssk-substrate-playground
cd ssk-substrate-playground
```

### 2. Build the project
```bash
cargo build
```

This will download all necessary dependencies, including actix-web, serde, and substrate-runtime-builder.

### 3. Install cargo-dotenv to run with enviroment files
```bash
cargo install cargo-dotenv
```
### 4. Run the Application
To start the server, run:

```bash
cargo dotenv -e .env.local run
```
### 5. Run the docker-compose
```bash
docker-compose up -d
```

The server will start at `http://127.0.0.1:3000`. You should see the following message in the console:

```bash 
Starting server at http://127.0.0.1:3000
```

The server is now running, and you can make API requests to generate and download Substrate projects.

### 6. Swagger
You can access the swagger openapi UI at [local](http://127.0.0.1:3000/docs), or [latest_release](https://dev-ssk.mvpw.io/api/docs)

## [API Reference](https://github.com/MVPWorkshop/ssk-substrate-playground/wiki/API-Reference)