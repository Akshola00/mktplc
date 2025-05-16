use crate::products::product_enums::{Category, ProductSize};
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Product {
    pub product_id: Uuid,
    pub name: String,
    pub description: String,
    pub original_price: f64,
    pub discount_percentage: f64,
    pub discount_price: Option<f64>,
    pub size: ProductSize,
    pub amount_in_stock: u64,
    pub available: bool,
    pub archived: bool,
    pub category: Category,
    pub seller_id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub tags: Vec<String>,
}

impl Product {
    pub fn new_product(
        name: String,
        description: String,
        original_price: f64,
        size: ProductSize,
        amount_in_stock: u64,
        category: Category,
        tags: Vec<String>,
    ) -> Self {
        let availability_status = if amount_in_stock > 0 { true } else { false };

        Product {
            product_id: Uuid::new_v4(),
            name,
            description,
            original_price,
            discount_percentage: 0.0,
            discount_price: None,
            size,
            amount_in_stock,
            available: availability_status,
            archived: false,
            category,
            seller_id: 2,
            created_at: Local::now(),
            updated_at: Local::now(),
            tags,
        }
    }
}
