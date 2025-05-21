use crate::products::product_struct::Product;

pub trait ValidateProductPerson {
    fn validate_product_adding(&self, product: Product) -> Result<(), String>;
}
