use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Album, NewAlbum};
use serde_json::Value;
use crate::models::AlbumData;

#[get("/albums", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let albums = Album::get_all_albums(&conn);
    Json(json!({
        "status": 200,
        "result": albums,
    }))
}

#[post["/albums", format = "application/json", data = "<new_album>"]]
pub fn new_album(conn: DbConn, new_album: Json<NewAlbum>) -> Json<Value> {
    Json(json!({
        "status": Album::insert_album(new_album.into_inner(), &conn),
        "result": Album::get_all_albums(&conn).first(),
    }))
}