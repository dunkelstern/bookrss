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
pub struct NarratorQueryParameters {
    language: Option<String>,
}

#[get("/narrators?<query>")]
pub fn get_narrator_list_filtered(query: NarratorQueryParameters, conn: DbConn) -> QueryResult<Json<Vec<Narrator>>> {
    let mut queryset = narrator::table.into_boxed::<DB>(); // this is so the queryset may be extended by additional filters below
    
    // language filter
    if let Some(language) = query.language {
        queryset = queryset.filter(narrator::language.eq(language));
    }

    queryset
        .order(narrator::id.asc())
        .load::<Narrator>(&*conn)
        .map(|narrator| Json(narrator))
}


#[get("/narrators")]
pub fn get_narrator_list(conn: DbConn) -> QueryResult<Json<Vec<Narrator>>> {
    // just return everything ordered by id
    narrator::table
        .order(narrator::name.asc())
        .load::<Narrator>(&*conn)
        .map(|narrator| Json(narrator))
}

#[get("/narrator/<id>")]
pub fn get_narrator(id: i32, conn: DbConn) -> Result<Json<Narrator>, Failure> {
    find_or_404!(narrator::table, Narrator, id, conn, |item| {
        Ok(Json(item))
    })
}

#[patch("/narrator/<id>", data="<data>")]
pub fn patch_narrator(id: i32, data: Json<Narrator>, conn: DbConn) -> Result<Json<Narrator>, Failure> {
    update_or_400!(narrator::table, Narrator, id, data, conn)
}

#[delete("/narrator/<id>")]
pub fn delete_narrator(id: i32, conn: DbConn) -> Result<Json<Narrator>, Failure> {
    // TODO: delete series, audiobooks and parts
    find_or_404!(narrator::table, Narrator, id, conn, |item| {
        let _ = delete(&item).execute(&*conn);

        Ok(Json(item))
    })
}

#[post("/narrator", data="<data>")]
pub fn create_narrator(data: Json<NewNarrator>, conn: DbConn) -> Result<Json<Narrator>, Failure> {
    let rows_inserted = insert_into(narrator::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .unwrap();
    
    if rows_inserted != 1 {
        Err(Failure(Status::InternalServerError))
    } else {
        let item = narrator::table
            .order(narrator::id.desc())
            .limit(1)
            .load::<Narrator>(&*conn)
            .unwrap().pop().unwrap();

        Ok(Json(item))
    }
}
