use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

use lib::settings::Settings;
use lib::database::{init_pool, DBConnection};

pub fn get_db_conn(settings: &Settings) -> PooledConnection<ConnectionManager<DBConnection>> {
    let pool = init_pool(settings.database.url.clone());
    
    pool.get().unwrap()
}
