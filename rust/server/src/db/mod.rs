
use rocket::{fairing::AdHoc, Rocket, Build};

use rocket_sync_db_pools::diesel;
use crate::diesel_migrations::{MigrationHarness, EmbeddedMigrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("db/migrations");

pub mod users;
pub mod credentials;
pub mod regstates;

#[database("db")]
pub struct Db(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| { c.run_pending_migrations(MIGRATIONS).unwrap(); }).await;
    rocket
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}