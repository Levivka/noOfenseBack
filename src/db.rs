use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};
use std::result::Result;

use crate::models::bid::Bid;
use crate::models::user::User;

pub struct Db {
    users: Collection<User>,
    bids: Collection<Bid>,
}

impl Db {
    pub async fn init(url: &str) -> Result<Self, Error> {
        match Client::with_uri_str(url).await {
            Ok(client) => Ok(Self {
                users: client.database("dnd-helper").collection("offenseUsers"),
                bids: client.database("dnd-helper").collection("offenseBids"),
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

    pub async fn login_check(&self, login: &String) -> Result<Option<User>, Error> {
        self.users
            .find_one(doc! {
                "login": login,
            })
            .await
    }

    pub async fn find_by_login_and_password(
        &self,
        login: &String,
        password: &String,
    ) -> Result<Option<User>, Error> {
        self.users
            .find_one(doc! {
                "login": login,
                "password": password
            })
            .await
    }

    pub async fn list_bids(&self) -> Result<Option<Vec<Bid>>, Error> {
        let mut cursor = self.bids.find(doc! {}).await?;
        let mut res: Vec<Bid> = vec![];

        while let Some(v) = cursor.try_next().await? {
            res.push(v);
        }

        match res.is_empty() {
            true => Ok(None),
            false => Ok(Some(res)),
        }
    }
    
    pub async fn user_bids(&self, user: &User) -> Result<Option<Vec<Bid>>, mongodb::error::Error> {
        let mut cursor = self.bids.find(doc! {
        }).await?;
        let mut res: Vec<Bid> = vec![];

        while let Some(v) = cursor.try_next().await? {
            res.push(v);
        }

        match res.is_empty() {
            true => Ok(None),
            false => Ok(Some(res)),
        }
    }

    // pub async fn bid_add(&self, bid: &Bid) -> Result<Option<Bid>, mongodb::error::Error> {
    //     self.bid_add(bid);
    // }
}
