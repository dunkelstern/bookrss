/// This macro tries to find an object in a table
/// the closure is called with the result or an error
/// is returned
///
/// $table: table object to query
/// $type: type to load
/// $id: id of object to find
/// $connection: DB connection to use
/// $closure: closure to execute with object
#[macro_export]
macro_rules! find_or_404 {
    ($table:expr, $type:ty, $id:expr, $connection:expr, $closure:expr) => {{
        let item = $table
            .find($id)
            .load::<$type>(&*$connection);

        // check if we could find the item
        match item {
            Ok(mut item) => {
                if let Some(item) = item.pop() {
                    // found it
                   $closure(item)
                } else {
                    // not found
                    Err(Failure(Status::NotFound))
                }
            },
            // DB error
            Err(_) => Err(Failure(Status::ServiceUnavailable))
        }
    }}
}

pub use diesel::update;

/// This macro tries to find an object in a table
/// to update it's content
///
/// $table: table object to query
/// $type: type to load
/// $id: id of object to find
/// $data: deserialized update to apply
/// $connection: DB connection to use
#[macro_export]
macro_rules! update_or_400 {
    ($table:expr, $type:ty, $id:expr, $data:expr, $connection:expr) => {{

        find_or_404!($table, $type, $id, $connection, |item: $type| {
            // found it
            if item.id == $data.id {
                let data = $data.into_inner();

                let _ = update(&item)
                    .set(&data)
                    .execute(&*$connection);

                Ok(Json(data))
            } else {
                Err(Failure(Status::BadRequest))
            }
        })
    }}
}

