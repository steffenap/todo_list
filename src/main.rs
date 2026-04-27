pub mod db;

use axum::{
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};


let app = Router::new()
    .route("/", get(home))
    .route("login", get(login))
    .route();

async fn home(){

}

async fn login(){

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    pub email: String,
    pub password: String,
}
    impl User {
        pub fn hash_password(&mut self){
            self.password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap()
        }

        pub fn verify_password(&self, password: &str) -> bool{
            bcrypt::verify(password, &self.password).unwrap()
        }
    }




#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}



