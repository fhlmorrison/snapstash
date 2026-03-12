use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::database::{self, AppState};
use std::path::PathBuf;
use actix_files::NamedFile;

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[post("/greet")]
async fn greet(req: web::Json<GreetRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}! You've been greeted from Rust Actix!", req.name))
}

#[derive(Deserialize)]
struct PathRequest {
    src: String,
}

#[post("/read_parameters")]
async fn read_parameters(req: web::Json<PathRequest>) -> impl Responder {
    let path = PathBuf::from(&req.src);
    let res = (|| {
        let decoder = png::Decoder::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
        let reader = decoder.read_info().map_err(|e| e.to_string())?;
        let chunks = &reader.info().uncompressed_latin1_text;
        chunks
            .iter()
            .find(|c| c.keyword == "parameters")
            .map(|c| c.text.clone())
            .ok_or("No parameters found".to_string())
    })();

    match res {
        Ok(params) => HttpResponse::Ok().json(params),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[post("/read_tags")]
async fn read_tags(state: web::Data<Arc<AppState>>, req: web::Json<PathRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::get_image_tags(db, &req.src) {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct TagImageRequest {
    image: String,
    tag: String,
}

#[post("/add_tag_to_image")]
async fn add_tag_to_image(state: web::Data<Arc<AppState>>, req: web::Json<TagImageRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::add_tag_to_image(db, &req.image, &req.tag) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/remove_tag_from_image")]
async fn remove_tag_from_image(state: web::Data<Arc<AppState>>, req: web::Json<TagImageRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::remove_tag_from_image(db, &req.image, &req.tag) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct SearchTagsRequest {
    tags: Vec<String>,
}

#[post("/search_with_tags")]
async fn search_with_tags(state: web::Data<Arc<AppState>>, req: web::Json<SearchTagsRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    let tags_refs: Vec<&str> = req.tags.iter().map(|s| s.as_str()).collect();
    match database::search_with_tags_and(db, tags_refs) {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct SearchTagsAdvancedRequest {
    positive_tags: Vec<String>,
    negative_tags: Vec<String>,
}

#[post("/search_with_tags_advanced")]
async fn search_with_tags_advanced(state: web::Data<Arc<AppState>>, req: web::Json<SearchTagsAdvancedRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    let pos_refs: Vec<&str> = req.positive_tags.iter().map(|s| s.as_str()).collect();
    let neg_refs: Vec<&str> = req.negative_tags.iter().map(|s| s.as_str()).collect();
    match database::search_with_tags_advanced(db, pos_refs, neg_refs) {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct AutoTagRequest {
    tag: String,
    images: Vec<String>,
    strict: bool,
}

#[post("/auto_tag")]
async fn auto_tag(state: web::Data<Arc<AppState>>, req: web::Json<AutoTagRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    
    // Auto tag logic (replicated from main.rs)
    let lower_tag = req.tag.to_lowercase();
    let lower_tag = lower_tag.as_str();

    for x in &req.images {
        let path = PathBuf::from(x);
        let params_res = (|| {
            let decoder = png::Decoder::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
            let reader = decoder.read_info().map_err(|e| e.to_string())?;
            let chunks = &reader.info().uncompressed_latin1_text;
            chunks
                .iter()
                .find(|c| c.keyword == "parameters")
                .map(|c| c.text.clone())
                .ok_or("No parameters found".to_string())
        })();

        if let Ok(p) = params_res {
            let tags = crate::parameters::get_prompts(&p);
            let contains = if req.strict {
                tags.contains(&lower_tag)
            } else {
                tags.iter().any(|t| t.to_lowercase().contains(lower_tag))
            };

            if contains {
                let _ = database::add_tag_to_image(db, x, &req.tag);
            }
        }
    }
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SaveImagesRequest {
    images: Vec<String>,
}

#[post("/save_images")]
async fn save_images(state: web::Data<Arc<AppState>>, req: web::Json<SaveImagesRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    
    for x in &req.images {
        let path = PathBuf::from(x);
        let params_res = (|| {
            let decoder = png::Decoder::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
            let reader = decoder.read_info().map_err(|e| e.to_string())?;
            let chunks = &reader.info().uncompressed_latin1_text;
            chunks
                .iter()
                .find(|c| c.keyword == "parameters")
                .map(|c| c.text.clone())
                .ok_or("No parameters found".to_string())
        })();

        match params_res {
            Ok(p) => { let _ = database::add_image_with_params(db, x, &p); },
            Err(_) => { let _ = database::add_image(db, x); },
        };
    }
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SearchImagesRequest {
    query_text: String,
}

#[post("/search_images")]
async fn search_images(state: web::Data<Arc<AppState>>, req: web::Json<SearchImagesRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::search_params(db, &req.query_text) {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_tags")]
async fn get_tags(state: web::Data<Arc<AppState>>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::get_tags(db) {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct CreateTagRequest {
    tag: String,
}

#[post("/create_tag")]
async fn create_tag(state: web::Data<Arc<AppState>>, req: web::Json<CreateTagRequest>) -> impl Responder {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().unwrap();
    match database::create_tag(db, &req.tag) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Serialize)]
struct ActixDirEntry {
    name: String,
    path: String,
    is_directory: bool,
    is_file: bool,
}

#[post("/read_dir")]
async fn read_dir(req: web::Json<PathRequest>) -> impl Responder {
    let path = PathBuf::from(&req.src);
    let entries = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let result: Vec<ActixDirEntry> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            Some(ActixDirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                is_directory: metadata.is_dir(),
                is_file: metadata.is_file(),
            })
        })
        .collect();

    HttpResponse::Ok().json(result)
}

#[get("/file/{path:.*}")]
async fn get_file(req: actix_web::HttpRequest) -> impl Responder {
    let path_str: String = req.match_info().query("path").parse().unwrap();
    let decoded_path = match urlencoding::decode(&path_str) {
        Ok(p) => p.into_owned(),
        Err(_) => return HttpResponse::BadRequest().body("Invalid path encoding"),
    };

    let path = PathBuf::from(decoded_path);
    if !path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    match NamedFile::open(path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn create_server(state: Arc<AppState>) -> std::io::Result<actix_web::dev::Server> {
    let state_data = web::Data::new(state);
    
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(state_data.clone())
            .service(greet)
            .service(read_parameters)
            .service(read_tags)
            .service(add_tag_to_image)
            .service(remove_tag_from_image)
            .service(search_with_tags)
            .service(search_with_tags_advanced)
            .service(auto_tag)
            .service(save_images)
            .service(search_images)
            .service(get_tags)
            .service(create_tag)
            .service(read_dir)
            .service(get_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
