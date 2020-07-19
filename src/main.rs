extern crate radarsoft_rust_contacts;

use radarsoft_rust_contacts::controllers::{github};
use actix_web::{web, HttpServer, App};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/contacts")
                .route("/github", web::get().to(github))
            )
    }).bind("127.0.0.1:8088")?.run().await
}