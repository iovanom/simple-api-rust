use crate::models::Movie;
use rocket::tokio;
use std::fs;

static MOVIES_DB: &str = "data/movies.json";

async fn _amovies() -> Result<Vec<Movie>, serde_json::Error> {}

fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);
    movies
}

fn _write_movies(movies: Vec<Movie>) {
    let data = serde_json::to_string(&movies).expect("Failed to convert movies into string");
    fs::write(MOVIES_DB, data).expect("Failed to write data");
}

pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        _ => None,
    }
}

pub fn get_by_rank(rank: String) -> Option<Movie> {
    match _movies() {
        Ok(movies) => movies
            .iter()
            .find(|movie| movie.rank == rank)
            .map(|movie| movie.clone()),
        _ => None,
    }
}

pub fn insert_movie(movie: Movie) -> Option<Movie> {
    match _movies() {
        Ok(mut movies) => {
            movies.push(movie.clone());
            _write_movies(movies);
            Some(movie)
        }
        _ => None,
    }
}

pub fn remove_by_rank(rank: String) -> Option<Movie> {
    match _movies() {
        Ok(movies) => {
            let movie = movies
                .iter()
                .find(|movie| movie.rank == rank)
                .map(|movie| movie.clone());
            if movie.is_some() {
                _write_movies(
                    movies
                        .iter()
                        .map(|movie| movie.to_owned())
                        .filter(|movie| movie.rank != rank)
                        .collect(),
                );
            }
            movie
        }
        _ => None,
    }
}
