use std::fmt;

use chrono::{Datelike, NaiveDate};

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {

    pub fn month(&self) -> u32 { self.month }
    
    pub fn day(&self) -> u32 { self.day }

    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
    pub fn from_str(s: &str) -> Self {

        let removed_dash_if_included = s.replace("-", "");
        assert!(removed_dash_if_included.len() == 4);

        let month_string = &removed_dash_if_included[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = removed_dash_if_included[2..].parse().unwrap(); 
        MonthDay { month, day }  
    }
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
    pub fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split("/").collect();
        if parts.len() < 2 {
            Category {
                primary: parts[0].to_string(),
                secondary: None,
            }
        } else {
            Category {
                primary: parts[0].to_string(),
                secondary: Some(parts[1].to_string()),
            }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}


#[derive(PartialEq, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub description: String,
    pub category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }
    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }

    pub fn category(&self) -> Category {
        self.category.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
            self.description,
            self.category
        )
    }
}
