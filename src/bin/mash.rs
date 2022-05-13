use mash_client::client::Client;

use chrono::{prelude::*};

const TOKEN: &str = "YOUR_TOKEN";
const PROFILE_ID: &str = "YOUR_PROFILE_ID";

fn main() {
    let client = Client::new(TOKEN, PROFILE_ID);

    // let monday = Utc::today() - Duration::days(Utc::today().weekday().number_from_monday() as i64);
    // let homework = client.get_homework(Some(monday), Some(monday+Duration::days(5))); // week
    let homework = client.get_homework_at(Utc::today().succ()); // tomorrow
    for work in homework {
        println!("{}: {}", work.subject_name, work.task);
    }
}
