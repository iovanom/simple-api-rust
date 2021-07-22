#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db;
use lib::models::Movie;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

/*
fn main() {
    rocket().launch();
}
*/

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

#[get("/<rank>")]
fn get_movie_by_rank(rank: &RawStr) -> Json<Option<Movie>> {
    Json(db::get_by_rank(
        rank.url_decode().expect("Failed to decode rank."),
    ))
}

#[delete("/<rank>")]
fn remove_movie_by_rank(rank: &RawStr) -> Json<Option<Movie>> {
    Json(db::remove_by_rank(
        rank.url_decode().expect("Failed to decode rank"),
    ))
}

#[post("/", data = "<movie>")]
fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::insert_movie(movie.0))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/movies",
        routes![
            get_movies,
            get_movie_by_rank,
            create_movie,
            remove_movie_by_rank
        ],
    )
}
