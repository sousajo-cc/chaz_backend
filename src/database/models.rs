#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Score {
    pub id: i32,
    pub high_score: i32,
    pub username: String,
}