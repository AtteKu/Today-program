use crate::events::{Category, Event};
use chrono::{Local, NaiveDate};
use crate::filters::EventFilter;

pub mod textfile;
pub mod csvfile;
pub mod sqlite;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
}

/*pub struct SimpleProvider {
    name: String,
}
impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let test_event = Event::new_singular(
            today,
            String::from("Test event for today"),
            Category::from_primary("test"),
        );
        events.push(test_event);
    }
} */
/*
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_provider_test_count() {
        let provider = SimpleProvider::new("Provider");
        
        let mut events = Vec::new();

        provider.get_events(&mut events);
        provider.get_events(&mut events);
        provider.get_events(&mut events);

        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_type() {
        let provider = SimpleProvider::new("Provider");

        let mut events = Vec::new();

        provider.get_events(&mut events);

        let event = &events[0];

        assert_eq!(event.description, "Test event for today");
        assert_eq!(event.category, Category::from_primary("test"));
    }
}
*/