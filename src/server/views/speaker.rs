use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use lib::database::DB;
use database::DbConn;

use lib::models::*;

#[derive(FromForm)]
pub struct SpeakerQueryParameters {
    language: Option<String>,
}

#[get("/speakers?<query>")]
pub fn get_speaker_list_filtered(query: SpeakerQueryParameters, conn: DbConn) -> QueryResult<Json<Vec<Speaker>>> {
    let mut queryset = speaker::table.into_boxed::<DB>(); // this is so the queryset may be extended by additional filters below
    
    // language filter
    if let Some(language) = query.language {
        queryset = queryset.filter(speaker::language.eq(language));
    }

    queryset
        .order(speaker::id.asc())
        .load::<Speaker>(&*conn)
        .map(|speaker| Json(speaker))
}


#[get("/speakers")]
pub fn get_speaker_list(conn: DbConn) -> QueryResult<Json<Vec<Speaker>>> {
    // just return everything ordered by id
    speaker::table
        .order(speaker::name.asc())
        .load::<Speaker>(&*conn)
        .map(|speaker| Json(speaker))
}

#[get("/speaker/<id>")]
pub fn get_speaker(id: i32, conn: DbConn) -> Result<Json<Speaker>, Failure> {
    find_or_404!(speaker::table, Speaker, id, conn, |item| {
        Ok(Json(item))
    })
}
