use models::*;
use chrono::prelude::*;

table! {
    part (id) {
        id -> Integer,
        import_date -> Timestamp,
        file_name -> Text,
        file_size -> Integer,
        start_time -> Integer,
        duration -> Integer,
        bit_rate -> Integer,
        audiobook_id -> Integer,
    }
}

joinable!(part -> audiobook (audiobook_id));

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, Serialize, Deserialize, Debug)]
#[table_name = "part"]
#[belongs_to(AudioBook)]
pub struct Part {
    pub id: i32,
    pub import_date: NaiveDateTime,
    pub file_name: String,
    pub file_size: i32,
    pub start_time: i32,
    pub duration: i32,
    pub bit_rate: i32,
    pub audiobook_id: i32,
}

#[derive(Insertable, Associations, Serialize, Deserialize, Debug)]
#[table_name = "part"]
#[belongs_to(AudioBook)]
pub struct NewPart {
    pub import_date: NaiveDateTime,
    pub file_name: String,
    pub file_size: i32,
    pub start_time: i32,
    pub duration: i32,
    pub bit_rate: i32,
    pub audiobook_id: i32,
}
