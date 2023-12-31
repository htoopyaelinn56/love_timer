use chrono::{self, *};

use std::io::Write;

use crate::password::Passwords;

mod password;

pub fn run() {
    print!("Please enter password: ");
    let password = Passwords::load_passwords_from_env();
    loop {
        std::io::stdout().flush().unwrap();
        let input_password = rpassword::read_password().unwrap();

        if password.check_password(&input_password) {
            count();
            break;
        } else {
            print!("Wrong Password. You can Enter Password Again:");
            continue;
        }
    }
}

fn count() {
    let today = Utc::now();
    let start_date = Utc.with_ymd_and_hms(2023, 6, 8, 9, 30, 0).unwrap();

    let diff = today.signed_duration_since(start_date);

    println!("{}", seconds_to_dhms(diff.num_seconds()));
}

fn seconds_to_dhms(seconds: i64) -> String {
    let days = seconds / 86400; // 86400 seconds in a day
    let remaining = seconds % 86400;

    let hours = remaining / 3600; // 3600 seconds in an hour
    let remaining = remaining % 3600;

    let minutes = remaining / 60; // 60 seconds in a minute
    let remaining_seconds = remaining % 60; // Remaining seconds

    format!(
        "We are together for {} days: {} hours: {} minutes: {} seconds.\nThoe Pout Lay ko chit tl {}",
        days,
        hours,
        minutes,
        remaining_seconds,
        "🩷 ".repeat(3)
    )
}
