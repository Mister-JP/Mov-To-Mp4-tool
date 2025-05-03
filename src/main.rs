use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures_util::TryStreamExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::Command;
use std::path::Path;
use std::fs;
use tera::{Tera, Context};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct ConversionResult {
    success: bool,
    message: String,
    original_filename: Option<String>,
    output_filename: Option<String>,
}

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("app_name", "MOV to MP4 Converter");
    context.insert("app_description", "Convert your MOV videos to MP4 format easily");
    
    match tmpl.render("index.html", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            error!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

// Helper function to get a user-friendly error message
fn get_user_friendly_error(error_message: &str) -> String {
    if error_message.contains("Output file does not contain any stream") {
        "The input file appears to be corrupted or doesn't contain valid video/audio streams".to_string()
    } else if error_message.contains("Invalid data found") {
        "The file contains invalid data or is corrupted".to_string()
    } else if error_message.contains("No such file or directory") {
        "FFmpeg failed to access the file. Make sure FFmpeg is properly installed.".to_string()
    } else if error_message.contains("Invalid argument") {
        "Invalid arguments passed to FFmpeg. The file might be corrupted.".to_string()
    } else {
        format!("Conversion failed: {}", error_message)
    }
}

async fn convert_video(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Create directories if they don't exist
    if !Path::new("uploads").exists() {
        fs::create_dir("uploads").unwrap();
    }
    if !Path::new("output").exists() {
        fs::create_dir("output").unwrap();
    }

    let mut original_filename = None;
    let mut temp_filepath = None;
    
    // Process multipart form data
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        
        if let Some(name) = content_disposition.get_name() {
            if name == "file" {
                let filename = content_disposition
                    .get_filename()
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| Uuid::new_v4().to_string());
                
                original_filename = Some(filename.clone());
                
                let filepath = format!("uploads/{}", filename);
                temp_filepath = Some(filepath.clone());
                
                // Save file
                let filepath_clone = filepath.clone();
                let mut file = web::block(move || std::fs::File::create(&filepath_clone))
                    .await?
                    .unwrap();
                
                while let Some(chunk) = field.try_next().await? {
                    // Clone file to move into closure
                    let mut file_clone = file;
                    file = web::block(move || file_clone.write_all(&chunk).map(|_| file_clone))
                        .await?
                        .unwrap();
                }
                
                info!("File saved: {}", filepath);
            }
        }
    }
    
    if let Some(filepath) = temp_filepath {
        if let Some(filename) = original_filename.clone() {
            // First, probe the file to check if it's valid
            let probe_output = Command::new("ffmpeg")
                .arg("-i")
                .arg(&filepath)
                .output();
                
            match probe_output {
                Ok(output) => {
                    let probe_result = String::from_utf8_lossy(&output.stderr);
                    info!("File probe result: {}", probe_result);
                    
                    // Check if the file is completely invalid
                    if probe_result.contains("Invalid data found") || probe_result.contains("does not contain any stream") {
                        let result = ConversionResult {
                            success: false,
                            message: "The file appears to be corrupted or is not a valid video file.".to_string(),
                            original_filename: Some(filename),
                            output_filename: None,
                        };
                        
                        return Ok(HttpResponse::BadRequest().json(result));
                    }
                    
                    // Generate output filename
                    let uuid = Uuid::new_v4();
                    let output_filename = format!("{}.mp4", uuid);
                    let output_filepath = format!("output/{}", output_filename);
                    
                    // Convert using ffmpeg with more robust parameters
                    let output = Command::new("ffmpeg")
                        .arg("-i")
                        .arg(&filepath)
                        .arg("-vcodec")
                        .arg("h264")
                        .arg("-acodec")
                        .arg("aac")
                        .arg("-strict")
                        .arg("-2")       // More lenient processing
                        .arg("-f")
                        .arg("mp4")      // Force output format
                        .arg("-y")       // Overwrite output files
                        .arg(&output_filepath)
                        .output();
                    
                    match output {
                        Ok(output) => {
                            if output.status.success() {
                                info!("Conversion successful: {} -> {}", filepath, output_filepath);
                                
                                let result = ConversionResult {
                                    success: true,
                                    message: "Conversion successful!".to_string(),
                                    original_filename: Some(filename),
                                    output_filename: Some(output_filename),
                                };
                                
                                return Ok(HttpResponse::Ok().json(result));
                            } else {
                                let error_message = String::from_utf8_lossy(&output.stderr);
                                error!("Conversion failed: {}", error_message);
                                
                                let user_friendly_message = get_user_friendly_error(&error_message);
                                
                                let result = ConversionResult {
                                    success: false,
                                    message: user_friendly_message,
                                    original_filename: Some(filename),
                                    output_filename: None,
                                };
                                
                                return Ok(HttpResponse::InternalServerError().json(result));
                            }
                        }
                        Err(e) => {
                            error!("Failed to execute FFmpeg: {}", e);
                            
                            let result = ConversionResult {
                                success: false,
                                message: format!("Failed to execute FFmpeg: {}", e),
                                original_filename: Some(filename),
                                output_filename: None,
                            };
                            
                            return Ok(HttpResponse::InternalServerError().json(result));
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to probe file: {}", e);
                    
                    let result = ConversionResult {
                        success: false,
                        message: format!("Failed to analyze file: {}. Is FFmpeg installed?", e),
                        original_filename: Some(filename),
                        output_filename: None,
                    };
                    
                    return Ok(HttpResponse::InternalServerError().json(result));
                }
            }
        }
    }
    
    let result = ConversionResult {
        success: false,
        message: "No file was uploaded".to_string(),
        original_filename: None,
        output_filename: None,
    };
    
    Ok(HttpResponse::BadRequest().json(result))
}

async fn download(req: web::Path<String>) -> impl Responder {
    let filename = req.into_inner();
    let filepath = format!("output/{}", filename);
    
    if Path::new(&filepath).exists() {
        actix_files::NamedFile::open(filepath)
            .map_err(|e| {
                error!("File error: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting server at http://127.0.0.1:8082");
    
    // Initialize templates
    let mut tera = Tera::default();
    tera.add_template_files(vec![
        ("src/templates/index.html", Some("index.html")),
    ]).unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/convert").route(web::post().to(convert_video)))
            .service(web::resource("/download/{filename}").route(web::get().to(download)))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(Files::new("/output", "./output").show_files_listing())
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}