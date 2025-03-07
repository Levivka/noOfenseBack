use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidDto {
    id: String,
    car_number: String,
    status: String,
    description: String,
}

impl BidDto {
    pub fn new(id: String, car_number: String, status: String, description: String) -> Self {
        Self {
            id,
            car_number,
            status,
            description,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn car_number(&self) -> &String {
        &self.car_number
    }

    pub fn set_car_number(&mut self, car_number: String) {
        self.car_number = car_number;
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
}
