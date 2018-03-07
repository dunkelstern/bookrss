use diesel::prelude::*;
use diesel::{delete, insert_into};

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use database::DbConn;

use lib::models::*;
use lib::macros::*;

#[derive(FromForm)]
pub struct AudioBooksQueryParameters {
    author_id: Option<i32>,
    series_id: Option<i32>,
    narrator_id: Option<i32>,
    translation: Option<String>,
}

#[get("/audiobooks?<query>")]
pub fn get_audiobook_list_filtered(query: AudioBooksQueryParameters, conn: DbConn) -> QueryResult<Json<Vec<AudioBook>>> {
    let mut queryset = audiobook::table
        .inner_join(series::table) // we probably need the author
        .into_boxed();             // this is so the queryset may be extended by additional filters below
    
    // author filter
    if let Some(author_id) = query.author_id {
        queryset = queryset.filter(series::author_id.eq(author_id));
    }

    // translation filter
    if let Some(translation) = query.translation {
        queryset = queryset.filter(series::translation.eq(translation));
    }

    // series filter
    if let Some(series_id) = query.series_id {
        queryset = queryset.filter(audiobook::series_id.eq(series_id));
    }

    // narrator filter
    if let Some(narrator_id) = query.narrator_id {
        queryset = queryset.filter(audiobook::narrator_id.eq(narrator_id));
    }

    // order by id, load from database
    let result = queryset
        .order(audiobook::id.asc())
        .load::<(AudioBook, Series)>(&*conn);
    
    // as we joined the series table above we have to split the
    // series object off again
    result.map(|books| 
        Json(
            books
                .into_iter()
                .map(|(book, _series)| book)
                .collect()
        )
    )
}

#[get("/audiobooks")]
pub fn get_audiobook_list(conn: DbConn) -> QueryResult<Json<Vec<AudioBook>>> {
    // just return everything ordered by id
    audiobook::table
        .order(audiobook::id.asc())
        .load::<AudioBook>(&*conn)
        .map(|books| Json(books))
}

#[get("/audiobook/<id>")]
pub fn get_audiobook(id: i32, conn: DbConn) -> Result<Json<AudioBook>, Failure> {
    find_or_404!(audiobook::table, AudioBook, id, conn, |item| {
        Ok(Json(item))
    })
}

#[patch("/audiobook/<id>", data="<data>")]
pub fn patch_audiobook(id: i32, data: Json<AudioBook>, conn: DbConn) -> Result<Json<AudioBook>, Failure> {
    update_or_400!(audiobook::table, AudioBook, id, data, conn)
}

#[delete("/audiobook/<id>")]
pub fn delete_audiobook(id: i32, conn: DbConn) -> Result<Json<AudioBook>, Failure> {
    // TODO: delete all parts belonging to the audiobook
    find_or_404!(audiobook::table, AudioBook, id, conn, |item| {
        let _ = delete(&item).execute(&*conn);

        Ok(Json(item))
    })
}

#[post("/audiobook", data="<data>")]
pub fn create_audiobook(data: Json<NewAudioBook>, conn: DbConn) -> Result<Json<AudioBook>, Failure> {
    let rows_inserted = insert_into(audiobook::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .unwrap();
    
    if rows_inserted != 1 {
        Err(Failure(Status::InternalServerError))
    } else {
        let item = audiobook::table
            .order(audiobook::id.desc())
            .limit(1)
            .load::<AudioBook>(&*conn)
            .unwrap().pop().unwrap();

        Ok(Json(item))
    }
}
