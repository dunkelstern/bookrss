use models::*;
    
table! {
    audiobook (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        part_no -> Integer,
        publish_date -> Nullable<Text>,
        narrator_id -> Integer,
        series_id -> Integer,
    }
}

joinable!(audiobook -> series (series_id));
joinable!(audiobook -> narrator (narrator_id));

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "audiobook"]
#[belongs_to(Narrator)]
#[belongs_to(Series)]
pub struct AudioBook {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub part_no: i32,
    pub publish_date: Option<String>,
    pub narrator_id: i32,
    pub series_id: i32,
}

#[derive(Insertable, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "audiobook"]
#[belongs_to(Narrator)]
#[belongs_to(Series)]
pub struct NewAudioBook {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub part_no: i32,
    pub publish_date: Option<String>,
    pub narrator_id: i32,
    pub series_id: i32,
}
