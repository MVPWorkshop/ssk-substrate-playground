use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self};
use std::path::{Path, PathBuf};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;
use zip::{write::FileOptions, ZipWriter};

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
            match ESupportedPallets::try_from(pallet.as_str()).unwrap_or(ESupportedPallets::Unknown)
            {
                ESupportedPallets::PalletUtility => {
                    pallets.push(ESupportedPallets::PalletUtility);
                }
                _ => continue,
            }
        }

        generate_project(project_name.clone(), pallets);
        format!("{} project generated successfully", project_name)
    })
    .await;

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
    let zip_file = fs::File::create(destination)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .large_file(true);

    // Walk through the directory and add files and subdirectories to the ZIP
    add_to_zip(&mut zip, source, source, &options)?;

    zip.finish()?;
    Ok(())
}

fn add_to_zip(
    zip: &mut ZipWriter<fs::File>,
    base_path: &Path,
    path: &Path,
    options: &FileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // Create the relative path for the ZIP file
        let relative_path = entry_path.strip_prefix(base_path).unwrap();

        // If the entry is a directory, recursively add its contents
        if entry_path.is_dir() {
            zip.add_directory(relative_path.to_string_lossy(), *options)?;
            add_to_zip(zip, base_path, &entry_path, options)?;
        } else {
            // Add the file to the ZIP
            zip.start_file(relative_path.to_string_lossy(), *options)?;
            let mut f = fs::File::open(&entry_path)?;
            io::copy(&mut f, zip)?;
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
            .route(
                "/download-project/{project_name}",
                web::get().to(download_project),
            )
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
