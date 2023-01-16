#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use sarina_web_server::services::{get_categories, get_products};
use sarina_web_server::services::{CategoryAllSerde, ProductSerde};

#[get("/")]
async fn categories() -> Json<CategoryAllSerde> {
    let cats = get_categories().await.unwrap();
    Json(cats)
}

#[get("/<category>")]
async fn products(category: i32) -> Json<Vec<ProductSerde>> {
    let prods = get_products(category).await.unwrap();
    Json(prods)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![categories, products])
}
