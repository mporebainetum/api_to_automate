use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::doc_model::Doc;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, //modify here
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    docs: Collection<Doc>,
    client: Client,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Database uri not set via MONGODB_URI env variable"),
        };

        let db_name = match env::var("MONGODB_DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Database name not set via MONGODB_DATABASE env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(db_name.as_str());
        let docs: Collection<Doc> = db.collection("docs");

        MongoRepo { docs, client }
    }

    pub fn create_doc(&self, new_doc: Doc) -> Result<InsertOneResult, Error> {
        let new_doc = Doc {
            id: None,
            info: new_doc.info,
        };
        let doc = self
            .docs
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating new doc");
        Ok(doc)
    }

    pub fn get_doc(&self, id: &String) -> Result<Doc, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let doc_detail = self
            .docs
            .find_one(filter, None)
            .ok()
            .expect("Error getting document");
        Ok(doc_detail.unwrap())
    }

    pub fn get_all_docs(&self) -> Result<Vec<Doc>, Error> {
        let cursors = self
            .docs
            .find(None, None)
            .ok()
            .expect("Error getting list of docs");
        let docs = cursors.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    pub fn delete_doc(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let doc_detail = self
            .docs
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting doc");
        Ok(doc_detail)
    }

    pub fn update_doc(&self, id: &String, new_doc: Doc) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_doc.id,
                    "info": new_doc.info,
                },
        };
        let updated_doc = self
            .docs
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating doc");
        Ok(updated_doc)
    }

    pub fn repo_state(&self) -> Result<Vec<String>, mongodb::error::Error> {
        self.client.list_database_names(None, None)
    }
}
