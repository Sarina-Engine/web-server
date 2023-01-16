use rocket::serde::Serialize;
use tonic::Request;
use web_server::web_server_client::WebServerClient;
use web_server::{Category, CategoryId, CategoryList, Empty, Product, ProductList};

pub mod web_server {
    tonic::include_proto!("web_server");
}

pub async fn get_categories() -> Result<CategoryAllSerde, Box<dyn std::error::Error>> {
    let mut client = WebServerClient::connect("http://[::1]:50055").await?;
    let category_vec = client
        .send_category(Request::new(Empty {}))
        .await?
        .into_inner()
        .category_vec;
    let mut main_cats: Vec<CategorySerde> = Vec::new();
    let mut side_cats: Vec<CategorySerde> = Vec::new();
    for category in category_vec {
        if category.parent_cat == 0 {
            main_cats.push(category.into());
        } else {
            side_cats.push(category.into());
        }
    }

    Ok(CategoryAllSerde {
        main_cats,
        side_cats,
    })
}

pub async fn get_products(cat_id: i32) -> Result<Vec<ProductSerde>, Box<dyn std::error::Error>> {
    let mut client = WebServerClient::connect("http://[::1]:50055").await?;
    let product_vec = client
        .send_product(Request::new(CategoryId { id: cat_id }))
        .await?
        .into_inner()
        .product_vec;
    let product_vec = product_vec.into_iter().map(|x| x.into()).collect();
    Ok(product_vec)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryAllSerde {
    pub main_cats: Vec<CategorySerde>,
    pub side_cats: Vec<CategorySerde>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CategorySerde {
    pub id: i32,
    pub code: String,
    pub title_fa: String,
    pub parent_cat: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductSerde {
    pub id: i32,
    pub name: String,
    pub rating: f64,
    pub emotion: f64,
    pub satisfaction: f64,
    pub recommended: f64,
    pub feeling: f64,
}

impl From<Product> for ProductSerde {
    fn from(p: Product) -> Self {
        let Product {
            id,
            name,
            rating,
            emotion,
            satisfaction,
            recommended,
            feeling,
        } = p;
        Self {
            id,
            name,
            rating,
            emotion,
            satisfaction,
            recommended,
            feeling,
        }
    }
}

impl From<Category> for CategorySerde {
    fn from(c: Category) -> Self {
        let Category {
            id,
            code,
            title_fa,
            parent_cat,
        } = c;
        Self {
            id,
            code,
            title_fa,
            parent_cat,
        }
    }
}
