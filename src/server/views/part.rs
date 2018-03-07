use std::path::Path;

use diesel::prelude::*;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::response::Failure;

use rocket_contrib::Json;

use config::Config;
use database::DbConn;

use lib::models::*;

#[get("/parts/<audiobook_id>")]
pub fn get_part_list(audiobook_id: i32, conn: DbConn) -> QueryResult<Json<Vec<Part>>> {
    // just return everything ordered by starting time
    part::table
        .filter(part::audiobook_id.eq(audiobook_id))
        .order(part::start_time.asc())
        .load::<Part>(&*conn)
        .map(|parts| Json(parts))
}


#[get("/part/<id>")]
pub fn get_part(id: i32, conn: DbConn, config: Config) -> Result<NamedFile, Failure> {
    find_or_404!(part::table, Part, id, conn, |item: Part| {
        let path = Path::new(&config.path.data_path)
            .join(item.file_name);

        NamedFile::open(&path)
            .map_err(|_| Failure(Status::NotFound))
    })
}

// TODO: write import api
// TODO: allow part deletion (delete file and cover too)
