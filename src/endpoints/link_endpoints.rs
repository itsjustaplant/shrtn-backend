use actix_web::{
    get, post,
    web::{Json, Path},
};
use mongodb::bson::doc;
use nanoid::nanoid;

use crate::models::link_models::{LinkPostModel, LinkPostResponseModel, LinkGetResponseModel};
use crate::db;

#[get("/{generated_url}")]
pub async fn get_link(
    path: Path<String>,
) -> Result<Json<LinkGetResponseModel>, Box<dyn std::error::Error>> {
    let filter = doc! { "generated_url": path.as_str() };
    let collection = db::get_collection().await?;
    if let Some(response) = collection.find_one(filter, None).await? {
        let response_data = LinkGetResponseModel {
            original_url: String::from(response.get_str("original_url")?),
            status_code: String::from("200"),
        };
        Ok(Json(response_data))
    } else {
        let response_data = LinkGetResponseModel {
            original_url: String::from("404 Not Found"),
            status_code: String::from("404"),
        };
        Ok(Json(response_data))
    }
}

#[post("/")]
pub async fn create_link(
    data: Json<LinkPostModel>,
) -> Result<Json<LinkPostResponseModel>, Box<dyn std::error::Error>> {
    let id = nanoid!(10);
    let doc = doc! { "original_url": data.original_url.clone(), "generated_url": id.clone()};
    let collection = db::get_collection().await?;
    collection.insert_one(doc, None).await?;
    let response = LinkPostResponseModel {
        generated_url: id.clone(),
    };

    Ok(Json(response))
}
