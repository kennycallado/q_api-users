use std::sync::Arc;
use tokio::sync::Mutex;

use super::claims::{Claims, UserInClaims};

pub struct Fetch {
    pub client: Arc<Mutex<reqwest::Client>>,
}

impl Fetch {
    pub fn new() -> Self {
        let client = Arc::new(Mutex::new(reqwest::Client::new()));
        Fetch { client }
    }

    pub async fn robot_token() -> Result<String, jsonwebtoken::errors::Error> {
        return Claims::from(UserInClaims::default()).enconde_for_robot();
    }
}
