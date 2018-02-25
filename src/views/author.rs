use diesel::prelude::*;

use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

use database::{DB, DbConn};

use models::author::{Author, author};

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
    let author = author::table
        .find(id)
        .load::<Author>(&*conn);

    // check if we could find the book
    match author {
        Ok(mut author) => {
            if let Some(author) = author.pop() {
                // found it
                Ok(Json(author))
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}
