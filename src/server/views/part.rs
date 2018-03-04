use std::path::Path;

use diesel::prelude::*;

use rocket::State;
use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::response::Failure;

use rocket_contrib::Json;

use server::DataPath;
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
pub fn get_part(id: i32, conn: DbConn, data_path: State<DataPath> ) -> Result<NamedFile, Failure> {
    let file = part::table
        .find(id)
        .select(part::file_name)
        .load::<String>(&*conn);

    if let Ok(mut file) = file {
        if let Some(file) = file.pop() {
            let path = Path::new(&data_path.0).join(file);
            NamedFile::open(&path).map_err(|_| Failure(Status::NotFound))
        } else {
            Err(Failure(Status::NotFound))
        }
    } else {
        Err(Failure(Status::ServiceUnavailable))        
    }
}
