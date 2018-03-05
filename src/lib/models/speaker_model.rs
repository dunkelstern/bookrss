table! {
    speaker (id) {
        id -> Integer,
        language -> Text,
        name -> Text,
        slug -> Text,
    }
}

#[derive(Queryable, Identifiable, Associations, Serialize, Debug)]
#[table_name = "speaker"]
pub struct Speaker {
    pub id: i32,
    pub language: String,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, Associations, Serialize, Debug)]
#[table_name = "speaker"]
pub struct NewSpeaker {
    pub language: String,
    pub name: String,
    pub slug: String,
}
