use chrono::{DateTime, UTC};
use crate::{db::DB, WebResult};
use serde::{Deserialize,Serialize};
use serde_json;
use warp::{http::StatusCode,reject,reply::json,Reply};


#[derive(Deserialize,Serialize,Debug)]
pub struct BookRequest{
    pub name : String,
    pub author: String,
    pub num_pages : usize,
    pub tags : Vec<String>,
}


pub async fn book_list_handler(db: DB)-> WebResult<impl Reply>{
    let books = db.fetch_books().await.map_err(|e| reject::custom(e))?;
    Ok(json(&books))
}

pub async fn get_book_handler(id:String , db:DB)-> WebResult<impl Reply>{
    db.get_book(id).wait.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn edit_book_handler(id:String , body:BookRequest,db:DB)-> WebResult<impl Reply>{
    db.edit_book(id,body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_book_handler(id:String, db:DB)->WebResult<impl Reply>{
    db.delete_book(id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}


pub async fn create_book_handler(body: BookRequest , db:DB)-> WebResult<impl Reply>{
    db.create_book(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}