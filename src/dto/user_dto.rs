use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDto {
    fio: String,
    login: String,
    password: String,

}

impl UserDto {
    
    pub fn fio(&self) -> &String {
        &self.fio
    }
    
    pub fn set_fio(&mut self, fio: String) {
        self.fio = fio;
    }
    
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
