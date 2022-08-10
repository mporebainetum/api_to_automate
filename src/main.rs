mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::doc_api::{create_doc, delete_doc, get_all_docs, get_doc, update_doc};
use api::health_api::get_health;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_doc])
        .mount("/", routes![get_doc])
        .mount("/", routes![get_all_docs])
        .mount("/", routes![delete_doc])
        .mount("/", routes![update_doc])
        .mount("/", routes![get_health])
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_test() {
        print!("Simple unit test which should be run before dockerizing!")
    }
}
