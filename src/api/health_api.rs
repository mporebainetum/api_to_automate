use crate::{models::health_model::Health, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/health")]
pub fn get_health(db: &State<MongoRepo>) -> Result<Json<Health>, Status> {
    match db.repo_state() {
        Ok(names) => Ok(Json(Health {
            state: String::from("UP"),
            databases: names.join(", "),
        })),
        Err(_) => Ok(Json(Health {
            state: String::from("DOWN"),
            databases: String::from("Failed to connect to database!"),
        })),
    }
}
