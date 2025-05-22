use uuid::Uuid;

use crate::products::product_struct::Product;

use super::person_enum::{PersonStatus, PersonType};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub role: PersonType,
    pub products_selling: Vec<Product>,
    pub buys: Vec<Product>,
    pub balance: f64,
    pub status: PersonStatus,
}

impl Person {
    pub fn new(name: String, role: PersonType) -> Self {
        Person {
            id: Uuid::new_v4(),
            name: name,
            role: role,
            products_selling: vec![],
            buys: vec![],
            balance: 10000.0,
            status: PersonStatus::ACTIVE,
        }
    }

    pub fn person_summary(&self) -> String {
        format!(
            "ID: {}\nName: {}\nRole: {:?}\nStatus: {:?}\nBalance: {}\n",
            self.id, self.name, self.role, self.status, self.balance
        )
    }
}
