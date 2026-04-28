use crate::db;
use crate::jwt::JwToken;
use crate::json_serialization::login::Login;
use crate::json_serialization::login_response::LoginResponse;

pub async fn login(credentials: web::Json<Login>, db: db) -> HttpResponse {
    let pass = credentials.password.clone();
    //TODO: FILTER USERS BY CREDENTIALS.USERNAME, AND TRY TO VERIFY PASS WITH USERNAME

    return match users[0].verify(pass) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            let response = LoginResponse{token: raw_token.clone()};
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().append_header(("token", raw_token)).json(&body)
        },
        false => HttpResponse::Unauthorized().finish()
    };
}