use crate::products::product_enums::{Category, ProductSize};
use crate::products::product_struct::Product;
use chrono::prelude::*;
use uuid::Uuid;
pub trait Getters {
    fn get_product_id(&self) -> Uuid;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_original_price(&self) -> f64;
    fn get_discount_percentage(&self) -> f64;
    fn get_discount_price(&self) -> Result<f64, String>;
    fn get_size(&self) -> ProductSize;
    fn get_amount(&self) -> u64;
    fn get_available(&self) -> bool;
    fn get_archived(&self) -> bool;
    fn get_category(&self) -> Category;
    fn get_seller_id(&self) -> u64;
    fn get_created_at(&self) -> DateTime<Local>;
    fn get_updated_at(&self) -> DateTime<Local>;
    fn get_tags(&self) -> Vec<String>;
    fn get_product_details(&self) -> String;
}

impl Getters for Product {
    fn get_product_id(&self) -> Uuid {
        self.product_id
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn get_original_price(&self) -> f64 {
        self.original_price
    }

    fn get_discount_percentage(&self) -> f64 {
        self.discount_percentage
    }
    fn get_discount_price(&self) -> Result<f64, String> {
        if self.discount_price.is_none() {
            return Err("No Discount on this product".to_owned());
        }
        Ok(self.discount_price.unwrap())
    }
    fn get_size(&self) -> ProductSize {
        self.size
    }
    fn get_amount(&self) -> u64 {
        self.amount_in_stock
    }
    fn get_available(&self) -> bool {
        self.available
    }
    fn get_archived(&self) -> bool {
        self.archived
    }
    fn get_category(&self) -> Category {
        self.category
    }
    fn get_seller_id(&self) -> u64 {
        self.seller_id
    }
    fn get_created_at(&self) -> DateTime<Local> {
        self.created_at
    }
    fn get_updated_at(&self) -> DateTime<Local> {
        self.updated_at
    }
    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
    fn get_product_details(&self) -> String {
        format!(
            "The product `{}` with an id of {} has the following attributes \n Product Description: {} \n Original Price: {}, Discount Percentage: {}, Discount Price: {:?}  \n Product Size: {:?} \n Availability Status: {} \n Seller: {} \n Categories: {:?} \n Tags: {:?}",
            &self.name,
            &self.product_id,
            &self.description,
            &self.original_price,
            &self.discount_percentage,
            &self.get_discount_price(),
            &self.size,
            &self.available,
            &self.seller_id,
            &self.category,
            &self.tags
        )
    }
}
