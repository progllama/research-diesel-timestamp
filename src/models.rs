use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub text: String,
    pub created_at: NaiveDateTime
}