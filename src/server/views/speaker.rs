use diesel::prelude::*;
use diesel::{delete, insert_into};

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use lib::database::DB;
use database::DbConn;

use lib::models::*;
use lib::macros::*;

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

#[patch("/speaker/<id>", data="<data>")]
pub fn patch_speaker(id: i32, data: Json<Speaker>, conn: DbConn) -> Result<Json<Speaker>, Failure> {
    update_or_400!(speaker::table, Speaker, id, data, conn)
}

#[delete("/speaker/<id>")]
pub fn delete_speaker(id: i32, conn: DbConn) -> Result<Json<Speaker>, Failure> {
    find_or_404!(speaker::table, Speaker, id, conn, |item| {
        let _ = delete(&item).execute(&*conn);

        Ok(Json(item))
    })
}

#[post("/speaker", data="<data>")]
pub fn create_speaker(data: Json<NewSpeaker>, conn: DbConn) -> Result<Json<Speaker>, Failure> {
    let rows_inserted = insert_into(speaker::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .unwrap();
    
    if rows_inserted != 1 {
        Err(Failure(Status::InternalServerError))
    } else {
        let item = speaker::table
            .order(speaker::id.desc())
            .limit(1)
            .load::<Speaker>(&*conn)
            .unwrap().pop().unwrap();

        Ok(Json(item))
    }
}
