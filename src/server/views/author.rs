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

#[patch("/author/<id>", data="<data>")]
pub fn patch_author(id: i32, data: Json<Author>, conn: DbConn) -> Result<Json<Author>, Failure> {
    update_or_400!(author::table, Author, id, data, conn)
}

#[delete("/author/<id>")]
pub fn delete_author(id: i32, conn: DbConn) -> Result<Json<Author>, Failure> {
    find_or_404!(author::table, Author, id, conn, |item| {
        let _ = delete(&item).execute(&*conn);

        Ok(Json(item))
    })
}

#[post("/author", data="<data>")]
pub fn create_author(data: Json<NewAuthor>, conn: DbConn) -> Result<Json<Author>, Failure> {
    let rows_inserted = insert_into(author::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .unwrap();
    
    if rows_inserted != 1 {
        Err(Failure(Status::InternalServerError))
    } else {
        let item = author::table
            .order(author::id.desc())
            .limit(1)
            .load::<Author>(&*conn)
            .unwrap().pop().unwrap();

        Ok(Json(item))
    }
}
