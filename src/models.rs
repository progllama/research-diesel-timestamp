use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime
}