use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod api_errors;
use self::api_errors::ApiError;

// TODO refactor url to environment variable
const BASE_URL: &str = include_str!("api_base_uri.txt");

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub data: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub token: String,
}


pub async fn create_account(username: String, password: String) -> AuthResponse {
    Request::post(&format!("{}/users", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
              "username": username,
              "password": password
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

pub async fn login(username: String, password: String) -> AuthResponse {
    Request::post(&format!("{}/users/login", BASE_URL))
        .header("content-type", "application/json")
        .body(
            json! ({
                "username": username,
                "password": password
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

pub async fn logout(token: &str) -> Result<(), ApiError> {
    let request = Request::post(&format!("{}/users/logout", BASE_URL))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();
    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}