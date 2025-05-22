use super::{product_enums::Category, product_struct::Product};

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
    fn validate_string(input: String) -> Result<(), String>;
    fn validate_tags(&mut self, tags: Vec<String>) -> Result<Vec<String>, String>;
    fn validate_pricing_rules(&self, price: f64) -> Result<(), String>;
    fn is_valid(&self) -> bool;
}

impl Validate for Product {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate_string(input: String) -> Result<(), String> {
        if input.is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        if input.len() > 100 {
            return Err("Product name cannot exceed 100 characters".to_string());
        }
        Ok(())
    }

    fn validate_tags(&mut self, tags: Vec<String>) -> Result<Vec<String>, String> {
        let existing_tags = &self.tags;
        if let Some(invalid_tag) = tags.iter().find(|tag| tag.len() > 20) {
            return Err(format!(
                "Invalid tag '{}', a tag length cannot be more than 20",
                invalid_tag
            ));
        }
        if tags.is_empty() {
            return Err("No tag passed".to_owned());
        };

        let clean_tags = tags
            .into_iter()
            .filter(|tag| !tag.trim().is_empty())
            .map(|tag| tag.to_lowercase());

        let mut correct_tags: Vec<String> = Vec::new();
        for tags in clean_tags {
            if !existing_tags.contains(&tags) {
                println!("china");
                correct_tags.push(tags);
            }
        }
        Ok(correct_tags)
    }

    fn validate_pricing_rules(&self, price: f64) -> Result<(), String> {
        if price < 1.0 {
            return Err("Price must not be less than 1.0".to_owned());
        } else if price > 99999.99 {
            return Err("Cannot exceed 100k".to_owned());
        }

        match self.category {
            Category::CLOTHING => {
                if price < 5000.0 {
                    return Err("Price of clothing must be greater than 5000".to_owned());
                }
            }
            Category::EDIBLE => {
                if price < 1000.0 {
                    return Err("Price of Edible must be greater than 3000".to_owned());
                }
            }
            Category::SHOES => {
                if price < 5000.0 {
                    return Err("Price of Shoes must be greater than 5000".to_owned());
                }
            }
            Category::AUTOMOBILE => {
                if price < 5000.0 {
                    return Err("Price of Automobile must be greater than 5000".to_owned());
                }
            }
            Category::OTHERS => return Ok(()),
            Category::ACCESSORIES => {
                if price < 5000.0 {
                    return Err("Price of Automobile must be greater than 5000".to_owned());
                }
            }
        }

        Ok(())
    }

    fn is_valid(&self) -> bool {
        let name_valid = Self::validate_string(self.name.clone()).is_ok();
        let description_valid = Self::validate_string(self.description.clone()).is_ok();
        let price_valid = self.validate_pricing_rules(self.original_price).is_ok();
        let amount_valid = self.amount_in_stock > 0;
        let tags_valid = self.tags.len() > 0 && self.tags.iter().all(|tag| tag.len() <= 20);

        name_valid && description_valid && price_valid && amount_valid && tags_valid
    }
}
