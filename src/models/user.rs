use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: String,
    fio: String,
    login: String,
    password: String,

}

impl User {
    pub fn new(id: String, fio: String, login: String, password: String) -> Self {
        Self { id, fio, login, password }
    }
    
    pub fn id(&self) -> &String {
        &self.id
    }
    
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    
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
