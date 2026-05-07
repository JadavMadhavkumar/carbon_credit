use crate::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResponse {
    pub id: String,
    pub name: String,
    pub bucket_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_accessed_at: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub path: String,
    pub idempotency_key: String,
}

pub struct SupabaseStorage {
    url: String,
    anon_key: String,
    service_key: Option<String>,
}

impl SupabaseStorage {
    pub fn new(url: &str, anon_key: &str, service_key: Option<String>) -> Self {
        Self {
            url: url.to_string(),
            anon_key: anon_key.to_string(),
            service_key,
        }
    }

    pub fn storage_url(&self) -> &str {
        &self.url
    }

    pub fn anon_key(&self) -> &str {
        &self.anon_key
    }

    pub fn service_key(&self) -> Option<&str> {
        self.service_key.as_deref()
    }

    pub async fn upload(
        &self,
        http: &reqwest::Client,
        bucket: &str,
        path: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<UploadResponse, AppError> {
        let url = format!("{}/storage/v1/object/{}/{}", self.url, bucket, path);

        let mut request = http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key))
            .header("Content-Type", content_type);

        if let Some(key) = &self.service_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request
            .body(content.to_vec())
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Storage upload failed: {}", e)))?;

        if !response.status().is_success() {
            let error: serde_json::Value = response.json().await.unwrap_or_default();
            return Err(AppError::Internal(
                error.get("message").and_then(|m| m.as_str()).unwrap_or("Upload failed").to_string()
            ));
        }

        let result: serde_json::Value = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse response: {}", e)))?;

        Ok(UploadResponse {
            path: result.get("path").and_then(|p| p.as_str()).unwrap_or(path).to_string(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
        })
    }

    pub async fn download(
        &self,
        http: &reqwest::Client,
        bucket: &str,
        path: &str,
    ) -> Result<Vec<u8>, AppError> {
        let url = format!("{}/storage/v1/object/{}/{}", self.url, bucket, path);

        let response = http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Storage download failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::NotFound(format!("File not found: {}/{}", bucket, path)));
        }

        let bytes = response.bytes().await
            .map_err(|e| AppError::Internal(format!("Failed to read file: {}", e)))?;

        Ok(bytes.to_vec())
    }

    pub async fn delete(
        &self,
        http: &reqwest::Client,
        bucket: &str,
        paths: &[String],
    ) -> Result<(), AppError> {
        let url = format!("{}/storage/v1/object/{}/remove", self.url, bucket);

        let body = serde_json::json!({ "paths": paths });

        let response = http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.service_key.as_ref().ok_or_else(|| AppError::Internal("Service key required for delete".to_string()))?))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Storage delete failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal("Delete operation failed".to_string()));
        }

        Ok(())
    }

    pub async fn get_public_url(&self, bucket: &str, path: &str) -> String {
        format!("{}/storage/v1/object/public/{}/{}", self.url, bucket, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_url_construction() {
        let storage = SupabaseStorage::new(
            "https://test.supabase.co",
            "anon_key",
            Some("service_key".to_string()),
        );
        
        let url = storage.get_public_url("waste-images", "user123/photo.jpg");
        assert!(url.contains("waste-images"));
        assert!(url.contains("photo.jpg"));
    }
}