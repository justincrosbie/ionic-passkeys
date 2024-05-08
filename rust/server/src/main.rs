#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
//#[macro_use] extern crate diesel;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use std::{env, fs};
use std::path::PathBuf;

use rocket::fs::{FileServer};

pub mod schema;

mod apis;
mod engine;
mod db;
mod cors;

use rocket::response::Redirect;
use dotenv::dotenv;

use crate::engine::startup::AppState;


#[launch]
fn rocket() -> _ {

    env_logger::init(); 
    println!("`env`");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    println!("`cwd`");
    match get_current_working_dir() {
        Ok(path) => println!("> {:?}", path),
        Err(why) => println!("! {:?}", why.kind()),
    }

    println!("`ls .`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    dotenv().ok();

    // Set DIST_FOLDER env var to be the path to the dist folder - either full path, or relative to the CWD
    // If running locally, ./rust/dist should do it
    let dist_folder = env::var("DIST_FOLDER").unwrap_or(".".to_string());
    let dist_path = PathBuf::from(dist_folder.clone());
    println!("dist_folder: {:?}", dist_path);

    let database_url = env::var("DATABASE_URL").unwrap_or(".".to_string());
    println!("database_url: {:?}", database_url);

    // let app_state = AppState::new();
    
    rocket::build()
    .attach(db::stage())
    .attach(cors::CORS)
    // .manage(app_state)
    .register("/", catchers![default])
    .attach(apis::stage())
    .mount("/", FileServer::from(dist_folder).rank(1))
}

#[catch(default)]
fn default() -> Redirect {
    Redirect::to(uri!("/"))

}


fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}