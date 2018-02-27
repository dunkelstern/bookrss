table! {
    author (id) {
        id -> Integer,
        language -> Text,
        name -> Text,
    }
}

#[derive(Queryable, Identifiable, Associations, Serialize, Debug)]
#[table_name = "author"]
pub struct Author {
    pub id: i32,
    pub language: String,
    pub name: String,
}
