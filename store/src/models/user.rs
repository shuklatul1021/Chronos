
use std::error::Error;

use diesel::prelude::*;
use uuid::Uuid;

use crate::helper::utils::get_user;
use crate::connect::Store;
use crate::schema::user;
use crate::structure::userstr::User;

impl Store {
    pub fn register_user(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, diesel::result::Error > {
        
        let user_uuid = Uuid::new_v4().to_string();
        let user_email = &email.clone();

        let u = User {
            id: user_uuid,
            username: username,
            email: email,
            password: password,
            created_at:  chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        // Find User If It Is Alredy Exit In DataBase
        let is_user_exists = get_user(user_email, &mut self.connection);
        if is_user_exists {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("User Already Exists".to_string()),
            ));
        }
    
        let result = diesel::insert_into(user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.connection)?;

        Ok(result)
    }

    pub fn login_user(
        &mut self,
        email: String,
        password: String
    ) -> Result<bool, diesel::result::Error> {
        // Implementation for user login
        Ok(false)
    }
}
