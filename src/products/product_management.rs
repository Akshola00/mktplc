use chrono::Local;

use super::product_validate::Validate;
use super::{product_enums::Category, product_struct::Product};

pub trait Manage {
    // Inventory operations
    fn increase_quantity(&mut self, amount: u64) -> Result<u64, String>;
    fn decrease_quantity(&mut self, amount: u64) -> Result<u64, String>;
    fn is_in_stock(&self) -> bool;

    // Pricing operations
    fn apply_discount(&mut self, percentage: f64) -> Result<f64, String>;
    fn remove_discount(&mut self) -> Result<(), String>;

    // Tag operations
    fn add_tag(&mut self, tag: String) -> Result<Vec<String>, String>;
    fn remove_tag(&mut self, tag: String) -> Result<Vec<String>, String>;

    // Search/filtering helpers
    fn matches_search(&self, query: String) -> bool;
    fn is_in_category(&self, category: &Category) -> bool;
    fn is_in_price_range(&self, min: f64, max: f64) -> bool;
}

impl Manage for Product {
    fn increase_quantity(&mut self, amount: u64) -> Result<u64, String> {
        self.amount_in_stock += amount;
        self.updated_at = Local::now();
        Ok(self.amount_in_stock)
    }

    fn decrease_quantity(&mut self, amount: u64) -> Result<u64, String> {
        self.amount_in_stock -= amount;
        self.updated_at = Local::now();
        Ok(self.amount_in_stock)
    }

    fn is_in_stock(&self) -> bool {
        self.amount_in_stock > 0
    }

    fn apply_discount(&mut self, percentage: f64) -> Result<f64, String> {
        if percentage >= 99.99 || percentage == 1.0 {
            return Err("Percentage must be between 100 and 0".to_owned());
        }

        self.discount_percentage = percentage;
        self.updated_at = Local::now();
        let discount: f64 = (percentage / 100.0) * self.original_price;
        self.discount_price = Some(discount);
        Ok(discount)
    }

    fn remove_discount(&mut self) -> Result<(), String> {
        if self.discount_percentage == 0.0 {
            return Err("No discount to remove".to_owned());
        };
        self.discount_price = None;
        self.discount_percentage = 0.0;
        Ok(())
    }

    fn add_tag(&mut self, tag: String) -> Result<Vec<String>, String> {
        let unique_tags = self.validate_tags(vec![tag])?;
        for tags in unique_tags {
            self.tags.push(tags);
        }
        todo!()
    }

    fn remove_tag(&mut self, tag: String) -> Result<Vec<String>, String> {
        let position = self.tags.iter().position(|x| *x == tag);

        match position {
            Some(index) => {
                self.tags.remove(index);
                Ok(self.tags.clone())
            }
            None => Err("Tag not found".to_string()),
        }
    }

    fn matches_search(&self, query: String) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
            || self
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&query))
    }

    fn is_in_category(&self, category: &Category) -> bool {
        let cat = self.category;
        cat == *category
    }

    fn is_in_price_range(&self, min: f64, max: f64) -> bool {
        match self.discount_price {
            Some(discounted) => discounted >= min && discounted <= max,
            None => self.original_price >= min && self.original_price <= max,
        }
    }
}
