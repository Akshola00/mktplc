use uuid::Uuid;

use crate::products::product_struct::Product;

use super::person_struct::Person;

pub trait ProductPersonImpl {
    fn create_product(&mut self, product: &Product) -> Result<(), String>;
    fn buy_a_product(&mut self, seller: &mut Person, product: Product) -> Result<(), String>;
    fn view_products(&self) -> Option<Vec<Product>>;
    fn update_product(&mut self, product_id: Uuid, product: Product) -> Result<(), String>;
    fn delete_product(&mut self, product_id: Uuid) -> Result<(), String>;
    fn get_product_by_id(&self, product_id: Uuid) -> Option<Product>;
    fn get_bought_products(&self) -> Option<Vec<Product>>;
}

impl ProductPersonImpl for Person {
    fn create_product(&mut self, product: &Product) -> Result<(), String> {
        self.products_selling.push(product.clone());
        Ok(())  
    }

    fn buy_a_product(&mut self, seller: &mut Person, product: Product) -> Result<(), String> {
        let product_index = seller
            .products_selling
            .iter()
            .position(|p| *p == product)
            .ok_or_else(|| "No product found with the seller".to_owned())?;

        let seller_product = &seller.products_selling[product_index];

        if seller_product.amount_in_stock <= 0 {
            return Err("Product not available - out of stock".to_owned());
        }

        if self.balance < seller_product.original_price {
            return Err("Not enough money to buy product".to_owned());
        }

        if !seller_product.available || seller_product.archived {
            // Added ! to available
            return Err("Product unavailable or archived by seller".to_owned());
        }

        self.balance -= seller_product.original_price;

        seller.balance += seller_product.original_price;

        let seller_product_mut = &mut seller.products_selling[product_index];
        seller_product_mut.amount_in_stock -= 1;

        let mut purchased_product = seller_product_mut.clone();
        purchased_product.amount_in_stock = 1; // Buyer bought 1 unit
        self.buys.push(purchased_product);

        if seller.products_selling[product_index].amount_in_stock == 0 {
            seller.products_selling.remove(product_index);
        }
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
    
    fn get_bought_products(&self) -> Option<Vec<Product>> {
        Some(self.buys.clone())
    }
}
