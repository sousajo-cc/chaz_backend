use diesel::table;
table! {
    score (id) {
        id -> Integer,
        high_score -> Integer,
        username -> Text,
    }
}
