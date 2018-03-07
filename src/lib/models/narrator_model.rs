table! {
    narrator (id) {
        id -> Integer,
        language -> Text,
        name -> Text,
        slug -> Text,
    }
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "narrator"]
pub struct Narrator {
    pub id: i32,
    pub language: String,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "narrator"]
pub struct NewNarrator {
    pub language: String,
    pub name: String,
    pub slug: String,
}
