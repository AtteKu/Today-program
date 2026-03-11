use chrono::NaiveDate;
use chrono::prelude::*;
use std::env;

pub fn handle_birthday() {
    let date_env = env::var("BIRTHDATE").expect("Could not find env variable");

    let user_birthday =
        NaiveDate::parse_from_str(&date_env, "%Y-%m-%d").expect("Could not convert to NaiveDate");

    let local = Local::now();
    let date_current: NaiveDate = local.date_naive();

    let month_user_birthday = user_birthday.month();
    let month_current = date_current.month();

    let day_user_birthday = user_birthday.day();
    let day_current = date_current.day();

    let year_user_birthday = user_birthday.year();
    let year_current = date_current.year();

    if month_user_birthday == month_current
        && day_user_birthday == day_current
        && year_current >= year_user_birthday
    {
        println!("Happy Birthday! \n");
    }

    let time_difference = (date_current - user_birthday).num_days();

    if time_difference > 0 {
        println!("You are {} days old. \n", time_difference);

        if time_difference % 1000 == 0 {
            println!("That's a nice round number! \n");
        }
    } else if time_difference < 0 {
        println!("Are you from the future? \n");
    } else {
        println!("Looks like you're new here. \n");
    }
}
