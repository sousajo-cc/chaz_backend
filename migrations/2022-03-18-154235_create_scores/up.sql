CREATE TABLE scores (
   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   high_score INTEGER NOT NULL,
   username TEXT NOT NULL,
   difficulty TEXT CHECK( difficulty IN ('TRAINING', 'EASY', 'MEDIUM', 'HARD', 'ZATOICHI') ) NOT NULL,
   level TEXT CHECK( level IN ('1', '2', '3') ) NOT NULL
)
