use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};
use std::result::Result;

use crate::models::user::User;

pub struct Db {
    users: Collection<User>,
}

impl Db {
    pub async fn init(url: &str) -> Result<Self, Error> {
        match Client::with_uri_str(url).await {
            Ok(client) => Ok(Self {
                users: client.database("dnd-helper").collection("offenseUsers"),
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn list_users(&self) -> Result<Option<Vec<User>>, Error> {
        let mut cursor = self.users.find(doc! {}).await?;
        let mut res: Vec<User> = vec![];

        while let Some(v) = cursor.try_next().await? {
            res.push(v);
        }

        match res.is_empty() {
            true => Ok(None),
            false => Ok(Some(res)),
        }
    }

    pub async fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        self.users.insert_one(user).await
    }

    pub async fn find_by_login_and_password(&self, login: &String, password: &String ) -> Result<Option<User>, Error> {
        self.users.find_one(doc! {
            "nickname": login,
            "password": password
        }).await
    }
}
