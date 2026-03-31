use std::collections::HashSet;

use crate::events::{Event, MonthDay, Category};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn accepts(&self, event: &Event) -> bool {
        if self.options.is_empty() {
            return true;
        }

        let mut results: Vec<bool> = Vec::new();

        for option in self.options.iter() {
            let result = match option {
                FilterOption::MonthDay(month_day) => {
                    *month_day == event.month_day()
                },
                FilterOption::Category(category) => {
                    *category == event.category()
                },
                FilterOption::Text(text) => {
                    event.description().contains(text)
                }
            };
            results.push(result);
        }
        // If the results vector contains only true values,
        // all the options match, and the event will be accepted,
        // otherwise it will be rejected by the filter.
        results.iter().all(|&option| option)
    }
}    

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.options.insert(FilterOption::Category(category));
        self
    }

    pub fn text(mut self, text: String) -> FilterBuilder {
        self.options.insert(FilterOption::Text(text));
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
        }
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;
    use chrono::{NaiveDate, Local, Datelike};

    #[test]
    fn filter_accepts_anything() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone());
        let filter = FilterBuilder::new()
        .build();
        assert!(filter.accepts(&event));
    }

    
    #[test]
    fn filter_rejects_one() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone());

        let filter = FilterBuilder::new()
            .text("queen".to_string())
            .category(rust_category)
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_one_accepts_two() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(1, 10))
            .text("king".to_string())
            .category(rust_category)
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_nothing() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(1, 10))
            .text("queen".to_string())
            .category(Category::new("Queens", "Rulers"))
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_all_parameters() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 25))
            .text("king".to_string())
            .category(Category::new("Kings", "Rulers"))
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn only_month_true() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 25))
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn only_description_true() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .text("king".to_string())
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn only_category_true() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .category(Category::new("Kings", "Rulers"))
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn only_one_but_false() {
        let rust_category = Category::new("Kings", "Rulers");

        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
            "Some king died".to_string(), 
            rust_category.clone()
        );


        let filter = FilterBuilder::new()
            .category(Category::new("Queens", "Rulers"))
            .build();
        assert!(!filter.accepts(&event));
    }

}
