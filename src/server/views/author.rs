use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use lib::database::DB;
use database::DbConn;

use lib::models::*;

#[derive(FromForm)]
pub struct AuthorQueryParameters {
    language: Option<String>,
}

#[get("/authors?<query>")]
pub fn get_author_list_filtered(query: AuthorQueryParameters, conn: DbConn) -> QueryResult<Json<Vec<Author>>> {
    let mut queryset = author::table.into_boxed::<DB>(); // this is so the queryset may be extended by additional filters below
    
    // language filter
    if let Some(language) = query.language {
        queryset = queryset.filter(author::language.eq(language));
    }

    queryset
        .order(author::id.asc())
        .load::<Author>(&*conn)
        .map(|author| Json(author))
}


#[get("/authors")]
pub fn get_author_list(conn: DbConn) -> QueryResult<Json<Vec<Author>>> {
    // just return everything ordered by id
    author::table
        .order(author::name.asc())
        .load::<Author>(&*conn)
        .map(|author| Json(author))
}

#[get("/author/<id>")]
pub fn get_author(id: i32, conn: DbConn) -> Result<Json<Author>, Failure> {
    find_or_404!(author::table, Author, id, conn, |item| {
        Ok(Json(item))
    })
}
