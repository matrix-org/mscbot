#![feature(never_type)]

#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate crypto;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate handlebars;
extern crate hex;
#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;
extern crate url;
extern crate urlencoded;
#[macro_use]
extern crate maplit;
extern crate itertools;

#[macro_use]
mod macros;

mod config;
mod domain;
mod error;
mod github;
mod nag;
mod scraper;
mod server;
mod teams;

use chrono::Local;
use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use env_logger::LogBuilder;
use log::LogRecord;

use config::CONFIG;

fn main() {
    // init environment variables, CLI, and logging
    dotenv::dotenv().ok();

    LogBuilder::new()
        .format(|rec: &LogRecord| {
            let loc = rec.location();
            format!("[{} {}:{} {}] {}",
                    rec.level(),
                    loc.module_path(),
                    loc.line(),
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    rec.args())
        })
        .parse(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init()
        .unwrap();

    debug!("Logging initialized.");
    let _ = CONFIG.check();
    let _ = DB_POOL.get().expect("Unable to test connection pool.");

    // we want to panic if we're unable to find any of the usernames
    let parsed_teams = teams::SETUP.team_labels().collect::<Vec<_>>();
    info!("parsed teams: {:?}", parsed_teams);

    // FIXME(anp) need to handle panics in both the listeners and crash the server
    let _ = scraper::start_scraping();
    let _server_handle = server::serve();

    // block
    //server_handle.join().expect("problem running server!").expect("problem while running server");
}

// initialize the database connection pool
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        info!("Initializing database connection pool.");

        let manager = ConnectionManager::<PgConnection>::new(CONFIG.db_url.clone());

        match Pool::builder()
            .max_size(CONFIG.db_pool_size)
            .build(manager)
        {
            Ok(p) => {
                info!("DB connection pool established.");
                p
            },
            Err(why) => {
                error!("Failed to establish DB connection pool: {}", why);
                panic!("Error creating connection pool.");
            }
        }
    };
}
