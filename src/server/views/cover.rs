use std::path::Path;
use std::str::FromStr;

use diesel::prelude::*;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Failure, Response, NamedFile, Responder};

use config::Config;
use database::DbConn;

use lib::models::*;


pub struct FileWithType(NamedFile);

impl<'r> Responder<'r> for FileWithType {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Content-Type", "image/jpeg")
            .ok()
    }
}

#[get("/cover/<id>")]
pub fn get_cover(id: String, conn: DbConn, config: Config) -> Result<FileWithType, Failure> {
    let id = i32::from_str(&id.trim_right_matches(".jpg")).unwrap();

    find_or_404!(part::table, Part, id, conn, |item: Part| {
        let path = Path::new(&config.path.data_path)
            .join(item.file_name)
            .with_extension("jpg");

        NamedFile::open(&path)
            .map(|f| FileWithType(f))
            .map_err(|_| Failure(Status::NotFound))
    })
}
