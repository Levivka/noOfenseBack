use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginDto {
    login: String,
    password: String,
}

impl LoginDto {
    pub fn login(&self) -> &String {
        &self.login
    }

    pub fn set_login(&mut self, login: String) {
        self.login = login;
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
