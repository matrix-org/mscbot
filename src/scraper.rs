use std::thread::{spawn, JoinHandle};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

use config::{CONFIG, GH_REPOS};
use github;

pub fn start_scraping() -> JoinHandle<()> {
    // spawn the github scraper in the background
    spawn(|| {
        let sleep_duration = Duration::from_secs(CONFIG.github_interval_mins * 60);
        loop {
            match github::most_recent_update() {
                Ok(gh_most_recent) => scrape_github(gh_most_recent),
                Err(why) => error!("Unable to determine most recent GH update: {:?}", why),
            }
            info!("GitHub scraper sleeping for {} seconds ({} minutes)",
                  sleep_duration.as_secs(),
                  CONFIG.github_interval_mins);
            thread::sleep(sleep_duration);
        }
    })
}

pub fn scrape_github(since: DateTime<Utc>) {
    info!("Scraping github activity since {:?}", since);
    let start_time = Utc::now().naive_utc();
    for repo in &GH_REPOS {
        match github::ingest_since(&repo, since) {
            Ok(_) => info!("Scraped {} github successfully", repo),
            Err(why) => error!("Unable to scrape github {}: {:?}", repo, why),
        }
    }

    ok_or!(github::record_successful_update(start_time), why =>
        error!("Problem recording successful update: {:?}", why));
}
