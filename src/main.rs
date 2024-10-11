use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: usize,
    name: String,
}
struct AppState {
    items: Mutex<HashMap<usize, Item>>,
    next_id: Mutex<usize>,
}
async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.values().cloned().collect::<Vec<Item>>())
}
async fn add_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();

    let new_item = Item {
        id: *next_id,
        name: item.name.clone(),
    };

    items.insert(new_item.id, new_item.clone());
    *next_id += 1;

    HttpResponse::Created().json(new_item)
}
async fn update_item(
    item: web::Json<Item>,
    item_id: web::Path<usize>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut items = data.items.lock().unwrap();

    if let Some(existing_item) = items.get_mut(&item_id.into_inner()) {
        existing_item.name = item.name.clone();
        HttpResponse::Ok().json(existing_item.clone())
    } else {
        HttpResponse::NotFound().body("Item not Found")
    }
}
async fn delete_item(item_id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();

    if items.remove(&item_id.into_inner()).is_some() {
        HttpResponse::Ok().body("Item dleted")
    } else {
        HttpResponse::NotFound().body("Item not Found")
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        items: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    });

    println!("Server running at http://127.0.0.1:8080/");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
