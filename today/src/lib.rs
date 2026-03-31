mod birthday;
mod events;
mod providers;
mod filters;

pub use events::{MonthDay, Event, Category};
pub use filters::{EventFilter, FilterBuilder};
pub use providers::EventProvider;

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
        pub name: String,
        pub kind: String,
        pub resource: Option<String>,
}

#[derive(Deserialize, Debug)]
    pub struct Config {
        pub providers: Vec::<ProviderConfig>,
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
            /*"simple" => {
                let provider = SimpleProvider::new(&cfg.name);
                providers.push(Box::new(provider));
            }, */
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

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter) -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();


    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(filter, &mut events); 
        /*let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name()); 
        count = new_count; */
    }
    
    for event in events {
        if filter.accepts(&event) {
            println!("{}", event);
        }
    }

    Ok(())
}

