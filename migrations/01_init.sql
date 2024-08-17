CREATE TABLE IF NOT EXISTS leaderboards (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS highscores (
    leaderboard_id BLOB NOT NULL,
    id BLOB PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    score BIGINT NOT NULL,
    FOREIGN KEY(leaderboard_id) REFERENCES leaderboards(id)
);

