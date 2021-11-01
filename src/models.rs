use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::albums;
use super::schema::albums::dsl::albums as all_albums;

#[derive(Serialize, Queryable)]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub year: String,
}

#[derive(Deserialize)]
pub struct AlbumData {
    pub artist: String,
}


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub title: String,
    pub artist: String,
    pub year: String,
}

impl Album {
    pub fn get_all_albums(conn: &PgConnection) -> Vec<Album> {
        all_albums
            .order(albums::id.desc())
            .load::<Album>(conn)
            .expect("error!")
    }

    
    pub fn insert_album(album: NewAlbum, conn: &PgConnection) -> Album {
        diesel::insert_into(albums::table)
            .values(&album)
            .get_result(conn)
            .expect("Error saving album")
    }
/*
    pub fn get_album_by_artist(album: AlbumData, conn: &PgConnection) -> Vec<Album> {
        all_albums
            .filter(albums::artist.eq(album.artist))
            .load::<Album>(conn)
            .expect("error!")
    }
*/
}
