table! {
    author (id) {
        id -> Integer,
        language -> Text,
        name -> Text,
        slug -> Text,
    }
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "author"]
pub struct Author {
    pub id: i32,
    pub language: String,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "author"]
pub struct NewAuthor {
    pub language: String,
    pub name: String,
    pub slug: String,
}
