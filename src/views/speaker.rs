use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use database::{DB, DbConn};

use models::speaker::{Speaker, speaker};

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
    let speaker = speaker::table
        .find(id)
        .load::<Speaker>(&*conn);

    // check if we could find the speaker
    match speaker {
        Ok(mut speaker) => {
            if let Some(speaker) = speaker.pop() {
                // found it
                Ok(Json(speaker))
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}
