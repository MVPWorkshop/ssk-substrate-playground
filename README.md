# Substrate Runtime Builder

This project is a web-based application built using Rust and Actix Web. It allows users to generate Substrate projects with selected pallets and download them as zip files. The project uses the `substrate-runtime-builder` crate to dynamically generate Substrate-based blockchain projects.

## Features

- Create a new Substrate project with selected pallets.
- Download the generated project as a zip file.
- Handle multiple requests concurrently using async execution with Actix Web.

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

### 3. Run the Application
To start the server, run:

```bash
cargo run
```

The server will start at http://127.0.0.1:8080. You should see the following message in the console:

```bash 
Starting server at http://127.0.0.1:8080
```

The server is now running, and you can make API requests to generate and download Substrate projects.


## API Endpoints

### 1. Generate a New Substrate Project
Endpoint: /generate-project

Method: POST

Request Body (JSON):

``` json
{
    "name": "your_project_name",
    "pallets": [
         "Assets",
        "Bounties",
        "Treasury",
        "Vesting",
        "Society",
        "Utility",
        "Identity",
        "Multisig",
        "Proxy",
        "Nfts",
        "Uniques",
        "Membership",
        "ChildBounties"
    ]
}
```

The name field specifies the name of the project, and the pallets field is an array of pallet names you want to include in the project.

### Example Request:

bash
```
curl -X POST http://127.0.0.1:8080/generate-project \
-H "Content-Type: application/json" \
-d '{"name": "my_project5", 
"pallets": [
            "Utility", 
            "Identity", 
            "Multisig", 
            "Proxy", 
            "Assets", 
            "Treasury", 
            "Vesting", 
            "Membership", 
            "Society"
            ], 
"push_to_git": false, 
"github_username": "username", 
"github_token": "Github_token", 
"github_email": "email"
}'
```

#### Response:

If the project is successfully created, you will receive the following message:

```
'my_project' project generated successfully.
```

### 2. Download the Generated Project
Endpoint: /download-project/{name}

Method: GET

After generating the project, you can download it as a zip file using this endpoint. Replace {name} with the name of the project you generated.

#### Example Request:

```bash
curl -o my_project.zip http://127.0.0.1:8080/download-project/my_project
```

This will download my_project.zip to your current directory.

#### Response:

If the project is found, you will receive the zip file for the project. If the project does not exist, you'll get a 404 Not Found response.
