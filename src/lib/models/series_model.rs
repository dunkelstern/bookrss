use models::*;

table! {
    series (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        translation -> Text,
        description -> Nullable<Text>,
        author_id -> Integer,
    }
}

joinable!(series -> author (author_id));

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "series"]
#[belongs_to(Author)]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub translation: String,
    pub description: Option<String>,
    pub author_id: i32,
}

#[derive(Insertable, Associations, FromForm, Serialize, Deserialize, Debug)]
#[table_name = "series"]
#[belongs_to(Author)]
pub struct NewSeries {
    pub title: String,
    pub slug: String,
    pub translation: String,
    pub description: Option<String>,
    pub author_id: i32,
}
