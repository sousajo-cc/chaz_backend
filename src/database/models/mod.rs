pub mod score;

pub mod tables {
    pub mod score {
        pub use crate::database::schema::scores;
        pub use crate::database::schema::scores::dsl::{
            high_score, id, scores as scores_table, username,
        };
    }
}
