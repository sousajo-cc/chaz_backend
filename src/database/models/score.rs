use crate::database::models::tables::score::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::cmp::Ordering;

#[derive(Queryable, Serialize, Deserialize, Identifiable, Insertable, Clone, Debug)]
#[table_name = "scores"]
#[primary_key(id)]
#[derive(Eq)]
pub struct Score {
    pub id: i32,
    pub high_score: i32,
    pub username: String,
    pub difficulty: String,
    pub level: String,
}

impl Score {
    pub fn insert(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(scores_table)
            .values(self)
            .execute(conn)
    }

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

impl PartialEq<Self> for Score {
    fn eq(&self, other: &Self) -> bool {
        self.high_score == other.high_score
    }
}

impl PartialOrd<Self> for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        other.high_score.cmp(&self.high_score)
    }
}
