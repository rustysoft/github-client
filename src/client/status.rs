use super::{Result, BASE_URL};
use crate::payload::BranchPayload;
use reqwest::Client;

pub struct StatusClient<'a> {
    pub(super) client: &'a Client,
}

impl<'a> StatusClient<'a> {
    pub fn get_combined(&self, repository_name: &str, r#ref: &str) -> Result<BranchPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/commits/{}/status",
                BASE_URL, repository_name, r#ref,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }
}
