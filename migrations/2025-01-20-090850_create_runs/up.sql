-- Your SQL goes here
CREATE TABLE timer_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user TEXT NOT NULL,
    working_time_secs INTEGER NOT NULL,  -- Store Duration as seconds
    breaking_time_secs INTEGER NOT NULL, -- Store Duration as seconds
    date DATE NOT NULL
);