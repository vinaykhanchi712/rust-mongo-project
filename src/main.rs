mod handler;
mod db;
mod errors;

use chrono::{DateTime, UTC};
use chrono::prelude::*;
use serde_json;
use serde::{Deserialize,Serialize};
use std::convert::Infallible;
use std::error;
use db::DB;

use warp::{Filter,Rejection};

type Result<T> = std::result::Result<T, dyn error::Error>;
type WebResult<T>= std::result::Result<T, Rejection>;


#[derive(Deserialize,Serialize,Debug)]
pub struct Book{
    pub id: String,
    pub name : String,
    pub author: String,
    pub num_pages : usize,
    pub added_at : DateTime<UTC>,
    pub tags : Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {

    let book= warp::path("book");
    let book_route= book
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_book_handler)
        .or(
            book
                .and(warp::put())
                .and(warp::path::param())
                .and(warp::body::json())
                .and(with_db(db.clone()))
                .and_then(handler::edit_book_handler))
        .or(book
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_book_handler))
        .or(book
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_book_handler));

    let routes = book_route.recover(errors::handle_rejection);
    println!("app started on port 8080");
    warp::serve(routes).run( [0,0,0,0] , 8080).await;
    Ok(())
}

fn with_db(db: DB)-> impl Filter<Extract=(DB,), Error = Infallible> + Clone{
    warp::any().map(move || db.clone())
}