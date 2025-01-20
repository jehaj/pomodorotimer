use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use chrono::prelude::*;
use crate::models::{NewTimerRun, TimerRuns};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_timer_run(conn: &mut SqliteConnection, user: &str, working_time_secs: &i32, breaking_time_secs: &i32) {
    use crate::schema::timer_runs;

    let local: NaiveDate = Local::now().date_naive();

    let new_run = NewTimerRun {
        user,
        working_time_secs,
        date: &local,
        breaking_time_secs,
    };
    
    diesel::insert_into(timer_runs::table)
        .values(&new_run)
        .execute(conn)
        .expect("Error saving new timer run");
}

pub fn get_timer_runs(conn: &mut SqliteConnection, username: &str) -> Vec<TimerRuns> {
    use crate::schema::timer_runs::dsl::*;

    let results = timer_runs
        .filter(user.eq(username))
        .select(TimerRuns::as_select())
        .load(conn)
        .expect("Error loading timer runs");

    results
}