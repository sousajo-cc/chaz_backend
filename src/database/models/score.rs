use crate::database::models::tables::score::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[derive(Queryable, Serialize, Identifiable, Insertable, Clone, Debug)]
#[table_name = "scores"]
#[primary_key(id)]
pub struct Score {
    pub id: i32,
    pub high_score: i32,
    pub username: String,
}

impl Score {
    pub fn insert_batch(conn: &SqliteConnection, values: Vec<Score>) -> QueryResult<usize> {
        diesel::insert_into(scores_table)
            .values(values)
            .execute(conn)
    }

    pub fn list(conn: &SqliteConnection) -> QueryResult<Vec<Score>> {
        scores_table.load::<Score>(conn)
    }

    pub fn find(conn: &SqliteConnection, name: String) -> QueryResult<Vec<Score>> {
        scores_table
            .filter(username.eq(name))
            .get_results::<Score>(conn)
    }
}
