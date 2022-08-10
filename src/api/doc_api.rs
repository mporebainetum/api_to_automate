use crate::{models::doc_model::Doc, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult}; //modify here
use rocket::{http::Status, serde::json::Json, State};

#[post("/doc", data = "<new_doc>")]
pub fn create_doc(
    db: &State<MongoRepo>,
    new_doc: Json<Doc>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Doc {
        id: None,
        info: new_doc.info.to_owned(),
    };
    let doc_detail = db.create_doc(data);
    match doc_detail {
        Ok(doc) => Ok(Json(doc)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/doc/<path>")]
pub fn get_doc(db: &State<MongoRepo>, path: String) -> Result<Json<Doc>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let doc_detail = db.get_doc(&id);
    match doc_detail {
        Ok(doc) => Ok(Json(doc)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/doc")]
pub fn get_all_docs(db: &State<MongoRepo>) -> Result<Json<Vec<Doc>>, Status> {
    let docs = db.get_all_docs();
    match docs {
        Ok(docs) => Ok(Json(docs)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/doc/<path>")]
pub fn delete_doc(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_doc(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Doc successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/doc/<path>", data = "<new_doc>")]
pub fn update_doc(
    db: &State<MongoRepo>,
    path: String,
    new_doc: Json<Doc>,
) -> Result<Json<Doc>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Doc {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        info: new_doc.info.to_owned(),
    };
    let update_result = db.update_doc(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_doc_info = db.get_doc(&id);
                return match updated_doc_info {
                    Ok(doc) => Ok(Json(doc)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
