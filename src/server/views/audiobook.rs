use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use database::DbConn;

use lib::models::*;

#[derive(FromForm)]
pub struct AudioBooksQueryParameters {
    author_id: Option<i32>,
    series_id: Option<i32>,
    speaker_id: Option<i32>,
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

    // speaker filter
    if let Some(speaker_id) = query.speaker_id {
        queryset = queryset.filter(audiobook::speaker_id.eq(speaker_id));
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
    let book = audiobook::table
        .find(id)
        .load::<AudioBook>(&*conn);

    // check if we could find the book
    match book {
        Ok(mut book) => {
            if let Some(book) = book.pop() {
                // found it
                Ok(Json(book))
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}
