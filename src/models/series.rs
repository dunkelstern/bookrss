use models::author::{author, Author};

table! {
    series (id) {
        id -> Integer,
        title -> Text,
        translation -> Text,
        description -> Nullable<Text>,
        author_id -> Integer,
    }
}

joinable!(series -> author (author_id));

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Debug)]
#[table_name = "series"]
#[belongs_to(Author)]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub translation: String,
    pub description: Option<String>,
    pub author_id: i32,
}
