use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bid {
    id: String,
    carNumber: String,
    status: String,
    description: String,
    // owner: User,
}

impl Bid {
    pub fn new(id: String, number: String, status: String, description: String) -> Self {
        Self {
            id,
            carNumber: number,
            status,
            description,
            // owner,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn car_number(&self) -> &String {
        &self.carNumber
    }

    pub fn set_car_number(&mut self, car_number: String) {
        self.carNumber = car_number;
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    // pub fn owner(&self) -> &User {
    //     &self.owner
    // }

    // pub fn set_owner(&mut self, owner: User) {
    //     self.owner = owner;
    // }
}
