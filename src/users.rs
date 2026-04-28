use rusqlite::Name;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Uuid)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

    impl User{
        pub fn new(username: String, email: String, password: String) -> User {
            let hashed: String = hash(password.as_str(), bcrypt::DEFAULT_COST).unwrap();
            let uuid = Uuid::new_v4().to_string();
            return User {
                username,
                email,
                password: hashed,
                unique_id: uuid
            }
        }
        pub fn verify(&self, password: &Str) -> bool {
            bcrypt::verify(password.as_str(), &self.password).unwrap()
        }
    }