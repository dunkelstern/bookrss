use r2d2;
use diesel::sqlite::{Sqlite, SqliteConnection};
use r2d2_diesel::ConnectionManager;

pub type DBConnection = SqliteConnection;
pub type DB = Sqlite;

type Pool = r2d2::Pool<ConnectionManager<DBConnection>>;

use rocket::Rocket;
use rocket::fairing::{Fairing, Kind, Info};

use lib::database::init_pool;

/// DB Pool fairing, adds the pool to the rocket instance to be used in the
/// connection guard
pub struct DbMiddleware;

impl Fairing for DbMiddleware {

    fn info(&self) -> Info {
        Info {
            name: "DB config",
            kind: Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        // load config
        let db_config = rocket.config().get_str("db").expect("db configuration in config file").to_string();

        // create pool
        let pool = init_pool(db_config);

        // attach pool to managed state
        Ok(rocket.manage(pool))
    }
}

//
// Request guard to access DB connection
//

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<DBConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &DBConnection.
impl Deref for DbConn {
    type Target = DBConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
