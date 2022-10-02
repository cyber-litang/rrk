use std::collections::HashMap;

use diesel::{connection::SimpleConnection, SqliteConnection};

mod model;
mod schema;

use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use model::NewUser;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
const DB_URL: &str = "rrk.db";

fn get_connection() -> Result<SqliteConnection> {
    let mut conn = SqliteConnection::establish(DB_URL)?;
    conn.batch_execute("PRAGMA busy_timeout = 5000")?;
    conn.batch_execute("PRAGMA journal_mode = WAL")?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(conn)
}

// class -> id
pub fn insert_users(infos: HashMap<String, Vec<String>>) -> Result<()> {
    use schema::users;
    let mut conn = get_connection()?;
    conn.transaction(|conn| {
        for (class, ids) in infos {
            for id in ids {
                let new_user = NewUser {
                    id: &id,
                    class: &class,
                };
                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(conn)?;
            }
        }
        Ok(())
    })
}

pub fn get_all_user_ids() -> Result<Vec<String>> {
    use schema::users::dsl::*;
    let mut conn = get_connection()?;
    let results = users.select(id).load::<String>(&mut conn)?;
    Ok(results)
}

pub fn get_user_ids_by_class(klass: &str) -> Result<Vec<String>> {
    use schema::users::dsl::*;
    let mut conn = get_connection()?;
    let results = users
        .select(id)
        .filter(class.eq(klass))
        .load::<String>(&mut conn)?;
    Ok(results)
}
