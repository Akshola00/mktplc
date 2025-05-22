use std::collections::HashMap;

use uuid::Uuid;

use crate::person::person_struct::Person;

pub struct Database_for_people(HashMap<Uuid, Person>);

impl Database_for_people {
    pub fn new() -> Database_for_people {
        Database_for_people(HashMap::new())
    }
    pub fn add_new_person(&mut self, id: Uuid, person: Person) {
        self.0.insert(id, person);
    }
}
