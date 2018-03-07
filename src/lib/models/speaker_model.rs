table! {
    speaker (id) {
        id -> Integer,
        language -> Text,
        name -> Text,
        slug -> Text,
    }
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "speaker"]
pub struct Speaker {
    pub id: i32,
    pub language: String,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "speaker"]
pub struct NewSpeaker {
    pub language: String,
    pub name: String,
    pub slug: String,
}
