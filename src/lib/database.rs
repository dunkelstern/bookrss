//
// DB Pool
//

use r2d2;
use diesel::sqlite::{Sqlite, SqliteConnection};
use r2d2_diesel::ConnectionManager;

pub type DBConnection = SqliteConnection;
pub type DB = Sqlite;

type Pool = r2d2::Pool<ConnectionManager<DBConnection>>;

pub fn init_pool(db_config: String) -> Pool {
    let manager = ConnectionManager::<DBConnection>::new(db_config);
    r2d2::Pool::new(manager).expect("db pool")
}

