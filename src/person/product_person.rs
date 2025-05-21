use uuid::Uuid;

use crate::products::product_struct::Product;

use super::person_struct::Person;

pub trait ProductPersonImpl {
    fn create_product(&mut self, product: Product) -> Result<(), String>;
    fn buy_a_product(&mut self, seller: Uuid, product: Product) -> Result<(), String>;
    fn view_products(&self) -> Option<Vec<Product>>;
    fn update_product(&mut self, product_id: Uuid, product: Product) -> Result<(), String>;
    fn delete_product(&mut self, product_id: Uuid) -> Result<(), String>;
    fn get_product_by_id(&self, product_id: Uuid) -> Option<Product>;
}

impl ProductPersonImpl for Person {
    fn create_product(&mut self, product: Product) -> Result<(), String> {
        self.products_selling.push(product);
        Ok(())
    }

    fn buy_a_product(&mut self, seller: Uuid, product: Product) -> Result<(), String> {
        
        Ok(())
    }

    fn view_products(&self) -> Option<Vec<Product>> {
        Some(self.products_selling.clone())
    }

    fn update_product(&mut self, product_id: Uuid, product: Product) -> Result<(), String> {
        let product_index = self
            .products_selling
            .iter()
            .position(|p| p.product_id == product_id);
        match product_index {
            Some(index) => {
                self.products_selling[index] = product;
            }
            None => return Err("Product not found".to_string()),
        }
        Ok(())
    }

    fn delete_product(&mut self, product_id: Uuid) -> Result<(), String> {
        let product_index = self
            .products_selling
            .iter()
            .position(|p| p.product_id == product_id);
        match product_index {
            Some(index) => {
                self.products_selling.remove(index);
            }
            None => return Err("Product not found".to_string()),
        }
        Ok(())
    }

    fn get_product_by_id(&self, product_id: Uuid) -> Option<Product> {
        self.products_selling
            .iter()
            .find(|p| p.product_id == product_id)
            .cloned()
    }
}
