use crate::net::models::{LoginResponse, PublicConfig, UploadResponse};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::json;

pub struct HavenApiClient {
    client: Client,
    base_url: String,
}

impl HavenApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    pub async fn get_public_config(&self) -> Result<PublicConfig, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/public-config", self.base_url);
        let resp = self.client.get(&url).send().await?;
        let config: PublicConfig = resp.json().await?;
        Ok(config)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/auth/login", self.base_url);
        let payload = json!({
            "username": username,
            "password": password,
            "eulaVersion": "2.0",
            "ageVerified": true
        });

        let resp = self.client.post(&url).json(&payload).send().await?;
        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(format!("Login failed: {}", err_text).into());
        }

        let login_res: LoginResponse = resp.json().await?;
        Ok(login_res)
    }

    pub async fn upload_file(
        &self,
        token: &str,
        file_name: &str,
        file_bytes: Vec<u8>,
    ) -> Result<UploadResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/upload-file", self.base_url);
        let part = Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = Form::new().part("file", part);

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(format!("Upload failed: {}", err_text).into());
        }

        let upload_res: UploadResponse = resp.json().await?;
        Ok(upload_res)
    }

    #[allow(dead_code)]
    pub async fn register(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/auth/register", self.base_url);
        let payload = json!({
            "username": username,
            "password": password,
            "eulaVersion": "2.0",
            "ageVerified": true
        });

        let resp = self.client.post(&url).json(&payload).send().await?;
        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(format!("Registration failed: {}", err_text).into());
        }

        let login_res: LoginResponse = resp.json().await?;
        Ok(login_res)
    }
}
