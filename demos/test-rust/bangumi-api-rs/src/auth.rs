use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use tokio::sync::oneshot;

/// ref: https://github.com/CircuitCoder/bgm.rs/blob/master/src/auth.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    pub access_token: String,
    pub user_id: u64,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppCred {
    pub client_id: String,
    pub client_secret: String,
}

impl AppCred {
    pub fn new(id: String, secret: String) -> AppCred {
        AppCred {
            client_id: id,
            client_secret: secret,
        }
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_client_secret(&self) -> &str {
        &self.client_secret
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "grant_type")]
pub enum AuthPayload {
    #[serde(rename = "authorization_code")]
    AuthorizationCode {
        #[serde(flatten)]
        app_cred: AppCred,
        code: String,
        redirect_uri: String,
        state: Option<String>,
    },

    #[serde(rename = "refresh_token")]
    RefreshToken {
        #[serde(flatten)]
        app_cred: AppCred,
        refresh_token: String,
        redirect_uri: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespError {
    error: String,
    error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuthResp {
    Success(AuthInfo),
    Error(RespError),
}

struct CodeService {
    sender: RefCell<Option<oneshot::Sender<String>>>,
}

pub async fn authorize(client_id: &str) {
    let port: u32 = 8989;
    let redirect = format!("http://localhost:{}/", port);
    let auth_uri = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}",
        "https://bgm.tv/oauth/authorize",
        client_id,
        redirect.clone()
    );
    println!("auth uri: {}", auth_uri);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let authInfo = AuthInfo {
            access_token: "token".to_string(),
            expires_in: 123213123,
            refresh_token: "refresh token".to_string(),
            user_id: 123112312,
        };
        let serialized = serde_json::to_string(&authInfo).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: AuthInfo = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);
    }
    #[test]
    fn test_auth() {
        // authorize("bgm191360de6815584eb");
    }
}
