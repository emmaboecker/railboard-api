use rocket::routes;

mod station_board;
pub use station_board::*;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![station_board::station_board]
}
