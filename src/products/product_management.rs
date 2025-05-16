use chrono::Local;

use super::{product_enums::Category, product_struct::Product};

pub trait Manage {
    // Inventory operations
    fn increase_quantity(&mut self, amount: u64) -> Result<u64, String>;
    fn decrease_quantity(&mut self, amount: u64) -> Result<u64, String>;
    fn is_in_stock(&self) -> bool;

    // Pricing operations
    fn apply_discount(&mut self, percentage: f64) -> Result<f64, String>;
    fn restore_original_price(&mut self) -> Result<f64, String>;

    // Status operations
    fn activate(&mut self) -> Result<(), String>;
    fn deactivate(&mut self) -> Result<(), String>;
    fn archive(&mut self) -> Result<(), String>;
    fn unarchive(&mut self) -> Result<(), String>;

    // Tag operations
    fn add_tag(&mut self, tag: String) -> Result<Vec<String>, String>;
    fn remove_tag(&mut self, tag: &str) -> Result<Vec<String>, String>;

    // Display and formatting
    fn display_summary(&self) -> String;
    fn format_for_catalog(&self) -> String;

    // Search/filtering helpers
    fn matches_search(&self, query: &str) -> bool;
    fn is_in_category(&self, category: &Category) -> bool;
    fn is_in_price_range(&self, min: f64, max: f64) -> bool;
}

impl Manage for Product {
    fn increase_quantity(&mut self, amount: u64) -> Result<u64, String> {
        self.amount_in_stock += amount;
        Ok(self.amount_in_stock)
    }

    fn decrease_quantity(&mut self, amount: u64) -> Result<u64, String> {
        self.amount_in_stock -= amount;
        Ok(self.amount_in_stock)
    }

    fn is_in_stock(&self) -> bool {
        self.amount_in_stock > 0
    }

    fn apply_discount(&mut self, percentage: f64) -> Result<f64, String> {
        // Discount Percentage = [(Original Price - Sale Price) / Original Price] * 100.
        if percentage >= 99.99 || percentage == 1.0 {
            return Err("Percentage must be between 100 and 0".to_owned());
        }

        self.discount_percentage = percentage;
        self.updated_at = Local::now();
        let discount: f64 = (percentage / 100.0) * self.original_price;
        self.discount_price = Some(discount);
        Ok(discount)
    }

    fn restore_original_price(&mut self) -> Result<f64, String> {
        todo!()
    }

    fn activate(&mut self) -> Result<(), String> {
        todo!()
    }

    fn deactivate(&mut self) -> Result<(), String> {
        todo!()
    }

    fn archive(&mut self) -> Result<(), String> {
        todo!()
    }

    fn unarchive(&mut self) -> Result<(), String> {
        todo!()
    }

    fn add_tag(&mut self, tag: String) -> Result<Vec<String>, String> {
        todo!()
    }

    fn remove_tag(&mut self, tag: &str) -> Result<Vec<String>, String> {
        todo!()
    }

    fn display_summary(&self) -> String {
        todo!()
    }

    fn format_for_catalog(&self) -> String {
        todo!()
    }

    fn matches_search(&self, query: &str) -> bool {
        todo!()
    }

    fn is_in_category(&self, category: &Category) -> bool {
        todo!()
    }

    fn is_in_price_range(&self, min: f64, max: f64) -> bool {
        todo!()
    }
}
