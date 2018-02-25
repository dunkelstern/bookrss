use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use database::{DB, DbConn};

use models::series::{Series, series};

#[derive(FromForm)]
pub struct SeriesQueryParameters {
    author_id: Option<i32>,
    translation: Option<String>,
}

#[get("/series?<query>")]
pub fn get_series_list_filtered(query: SeriesQueryParameters, conn: DbConn) -> QueryResult<Json<Vec<Series>>> {
    let mut queryset = series::table.into_boxed::<DB>(); // this is so the queryset may be extended by additional filters below
    
    // author filter
    if let Some(author_id) = query.author_id {
        queryset = queryset.filter(series::author_id.eq(author_id));
    }

    // translation filter
    if let Some(translation) = query.translation {
        queryset = queryset.filter(series::translation.eq(translation));
    }

    queryset
        .order(series::id.asc())
        .load::<Series>(&*conn)
        .map(|series| Json(series))
}

#[get("/series")]
pub fn get_series_list(conn: DbConn) -> QueryResult<Json<Vec<Series>>> {
    // just return everything ordered by id
    series::table
        .order(series::id.asc())
        .load::<Series>(&*conn)
        .map(|series| Json(series))
}

#[get("/series/<id>")]
pub fn get_series(id: i32, conn: DbConn) -> Result<Json<Series>, Failure> {
    let series = series::table
        .find(id)
        .load::<Series>(&*conn);

    // check if we could find the book
    match series {
        Ok(mut series) => {
            if let Some(series) = series.pop() {
                // found it
                Ok(Json(series))
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}
