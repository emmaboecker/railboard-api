use error_handlers::not_found;
use rocket::catchers;

mod error_handlers;

mod server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .register("/", catchers![not_found])
        .mount("/vendo/v1/", server::vendo::get_routes())
        .launch()
        .await?;

    Ok(())
}
