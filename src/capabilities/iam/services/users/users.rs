use super::super::super::*;
use entities::users::{self, Entity as User, Status};
use migration::sea_orm;
use sea_orm::prelude::Uuid;
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm::ColumnTrait;

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn find_user_by_email(&self, email: String) -> Result<Option<users::Model>, DbErr> {
        User::find().filter(users::Column::Email.eq(email)).one(&self.db).await
    }
    pub async fn find_active_user_by_email(&self, email: String) -> Result<Option<users::Model>, DbErr> {
        User::find().filter(users::Column::Email.eq(email)).filter(users::Column::Status.eq(Status::Active)).one(&self.db).await
    }

    pub async fn create_user(&self, email: String, first_name: String, last_name: String) -> Result<Option<users::Model>, DbErr> {
        let existing_user_result = &self.find_user_by_email(email.clone()).await;
        match existing_user_result {
            Ok(Some(_)) => return Err(DbErr::Custom(String::from("Already Exists"))),
            _ => (),
        }

            let user = users::ActiveModel {
                email: Set(email.to_string()),
                first_name: Set(first_name.to_string()),
                last_name: Set(last_name.to_string()),
                status: Set(users::Status::Active),
                pid: Set(Uuid::new_v4()),
                ..Default::default()
            };

            let res = User::insert(user).exec_with_returning(&self.db).await;
            match res {
                Ok(user) => return Ok(Some(user)),
                Err(e) => return Err(e),
            }
    }

    pub async fn find_user_by_pid(&self, pid: Uuid) -> Result<Option<users::Model>, DbErr> {
        User::find().filter(users::Column::Pid.eq(pid)).one(&self.db).await
    }
}