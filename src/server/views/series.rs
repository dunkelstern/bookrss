use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use lib::database::DB;
use database::DbConn;

use lib::models::*;

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
    find_or_404!(series::table, Series, id, conn, |item| {
        Ok(Json(item))
    })
}
