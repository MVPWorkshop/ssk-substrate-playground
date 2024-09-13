use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;
use std::path::{Path, PathBuf};
use zip::{ZipWriter, write::FileOptions};
use std::fs::{self, File};
use std::io::{self, Write};

// Define a struct for the project with a vector of pallets
#[derive(Serialize, Deserialize)]
struct NewProject {
    name: String,
    pallets: Vec<String>,
}

// A function to greet a user by their name (path parameter)
async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello, {}!", name))
}

// A function to create a new project with a list of pallets
async fn generate_a_project(project: web::Json<NewProject>) -> impl Responder {
    let project_name = project.name.clone();
    let pallet_names = project.pallets.clone();

    let result = actix_web::web::block(move || {
        let mut pallets: Vec<ESupportedPallets> = Vec::new();

        for pallet in &pallet_names {
            match ESupportedPallets::try_from(pallet.as_str()).unwrap_or(ESupportedPallets::Unknown) {
                ESupportedPallets::PalletUtility => {
                    pallets.push(ESupportedPallets::PalletUtility);
                }
                _ => continue,
            }
        }

        generate_project(project_name.clone(), pallets);
        format!("{} project generated successfully", project_name)
    }).await;

    match result {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(_) => HttpResponse::InternalServerError().body("Error generating the project"),
    }
}

// A function to download the generated project as a ZIP file
async fn download_project(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let project_name = path.into_inner();
    let project_path = PathBuf::from(format!("./generated_code/{}", project_name)); // Adjust the path as needed

    if project_path.is_dir() {
        let zip_path = format!("./generated_code/{}.zip", project_name);
        if let Err(e) = zip_directory(&project_path, &zip_path) {
            eprintln!("Error creating ZIP: {}", e); // Log the error to the console
            return HttpResponse::InternalServerError().body(format!("Error creating ZIP: {}", e));
        }

        match NamedFile::open(zip_path) {
            Ok(file) => file.into_response(&req),
            Err(e) => {
                eprintln!("Error opening ZIP file: {}", e); // Log the error to the console
                HttpResponse::InternalServerError().body("Error opening ZIP file")
            }
        }
    } else {
        eprintln!("Project directory not found: {}", project_path.display()); // Log the missing directory
        HttpResponse::NotFound().body("Project not found")
    }
}

// Function to create a ZIP file from a directory
fn zip_directory(source: &Path, destination: &str) -> io::Result<()> {
    let zip_file = File::create(destination)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .large_file(true);

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let name = path.strip_prefix(source).unwrap();
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(&path)?;
            io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
            .route("/download-project/{project_name}", web::get().to(download_project))
    })
        .workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}