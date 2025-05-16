use crate::products::product_enums::{Category, ProductSize};
use crate::products::product_struct::Product;
use chrono::Local;
pub trait Setters {
    fn set_name(&mut self, name: String) -> Result<(), String>;
    fn set_description(&mut self, description: String) -> Result<(), String>;
    fn set_price(&mut self, price: f64) -> Result<(), String>;
    fn set_size(&mut self, size: ProductSize) -> Result<(), String>;
    fn set_amount(&mut self, amount: u64) -> Result<(), String>;
    fn set_available(&mut self, available: bool) -> Result<(), String>;
    fn set_archived(&mut self, archived: bool) -> Result<(), String>;
    fn set_category(&mut self, category: Category) -> Result<(), String>;
    fn set_tags(&mut self, tags: Vec<String>) -> Result<(), String>;
    fn set_discount_price(&mut self, price: f64) -> Result<(), String>;
}

impl Setters for Product {
    fn set_name(&mut self, name: String) -> Result<(), String> {
        self.name = name;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_description(&mut self, description: String) -> Result<(), String> {
        self.description = description;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_price(&mut self, price: f64) -> Result<(), String> {
        self.original_price = price;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_discount_price(&mut self, price: f64) -> Result<(), String> {
        self.discount_price = Some(price);
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_size(&mut self, size: ProductSize) -> Result<(), String> {
        self.size = size;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_amount(&mut self, amount: u64) -> Result<(), String> {
        self.amount_in_stock = amount;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_available(&mut self, available: bool) -> Result<(), String> {
        self.available = available;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_archived(&mut self, archived: bool) -> Result<(), String> {
        self.archived = archived;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_category(&mut self, category: Category) -> Result<(), String> {
        self.category = category;
        self.updated_at = Local::now();
        Ok(())
    }
    fn set_tags(&mut self, tags: Vec<String>) -> Result<(), String> {
        self.tags = tags;
        self.updated_at = Local::now();
        Ok(())
    }
}
