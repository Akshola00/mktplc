use uuid::Uuid;

use crate::products::product_struct::Product;

use super::person_enum::{PersonStatus, PersonType};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub role: PersonType,
    pub products_selling: Vec<Product>,
    pub balance: f64,
    pub status: PersonStatus,
    pub dummy_money: f64,
}

impl Person {
    pub fn new(name: String, role: PersonType) -> Self {
        Person {
            id: Uuid::new_v4(),
            name: name,
            role: role,
            products_selling: vec![],
            balance: 10000.0,
            status: PersonStatus::ACTIVE,
            dummy_money: 200.0,
        }
    }

    pub fn person_summary(&self) -> String {
        format!(
            "The person {}
         has an id of {} and, Status {:?}. 
         On This Market place he 
         is a {:?}, All products/buys {:#?}",
            self.name, self.id, self.status, self.role, self.products_selling
        )
    }
}
