mod birthday;
mod events;
mod providers;
mod filters;


use crate::events::{Category, Event, MonthDay};
use crate::providers::{EventProvider, SimpleProvider};
use crate::providers::{
    csvfile::CSVFileProvider,
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
};

use chrono::{Datelike, Local, NaiveDate};
use std::error::Error;
use std::path::Path;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
    pub struct ProviderConfig {
        name: String,
        kind: String,
        resource: Option<String>,
}

#[derive(Deserialize, Debug)]
    pub struct Config {
        providers: Vec::<ProviderConfig>,
}


fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    
    for cfg in config.providers.iter() {

        match cfg.kind.as_str() {
            "text" => {
                let resource = cfg.resource.as_ref()
                    .expect("Text provider needs a resource");

                let path = config_path.join(resource);
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let resource = cfg.resource.as_ref()
                    .expect("CSV provider needs a resource");

                let path = config_path.join(resource);
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "simple" => {
                let provider = SimpleProvider::new(&cfg.name);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let resource = cfg.resource.as_ref()
                    .expect("Sqlite provider needs a resource");

                let path = config_path.join(resource);
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            },      
        }
    }
    providers
}

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events); 
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }
    
    for event in events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }

    Ok(())
}

