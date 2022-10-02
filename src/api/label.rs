//! Labels

use crate::models::{LabelCreateRequest, LabelResponse, LabelUpdate, LabelsResponse};
use crate::{Client, HttpSnafu, RequestError, ReqwestProcessingSnafu, SerializingSnafu};
use reqwest::{Method, StatusCode};
use snafu::ResultExt;
use std::collections::HashMap;

impl Client {
    /// List all Labels
    pub async fn labels(&self) -> Result<LabelsResponse, RequestError> {
        Ok(self.get_labels(None).await?)
    }

    /// List all Labels by organization ID
    pub async fn labels_by_org(&self, org_id: &str) -> Result<LabelsResponse, RequestError> {
        Ok(self.get_labels(Some(org_id)).await?)
    }

    async fn get_labels(&self, org_id: Option<&str>) -> Result<LabelsResponse, RequestError> {
        let labels_url = format!("{}/api/v2/labels", self.url);
        let mut request = self.request(Method::GET, &labels_url);

        if let Some(id) = org_id {
            request = request.query(&[("orgID", id)]);
        }

        let response = request.send().await.context(ReqwestProcessingSnafu)?;
        match response.status() {
            StatusCode::OK => Ok(response
                .json::<LabelsResponse>()
                .await
                .context(ReqwestProcessingSnafu)?),
            status => {
                let text = response.text().await.context(ReqwestProcessingSnafu)?;
                HttpSnafu { status, text }.fail()?
            }
        }
    }

    /// Retrieve a label by ID
    pub async fn find_label(&self, label_id: &str) -> Result<LabelResponse, RequestError> {
        let labels_by_id_url = format!("{}/api/v2/labels/{}", self.url, label_id);
        let response = self
            .request(Method::GET, &labels_by_id_url)
            .send()
            .await
            .context(ReqwestProcessingSnafu)?;
        match response.status() {
            StatusCode::OK => Ok(response
                .json::<LabelResponse>()
                .await
                .context(ReqwestProcessingSnafu)?),
            status => {
                let text = response.text().await.context(ReqwestProcessingSnafu)?;
                HttpSnafu { status, text }.fail()?
            }
        }
    }

    /// Create a Label
    pub async fn create_label(
        &self,
        org_id: &str,
        name: &str,
        properties: Option<HashMap<String, String>>,
    ) -> Result<LabelResponse, RequestError> {
        let create_label_url = format!("{}/api/v2/labels", self.url);
        let body = LabelCreateRequest {
            org_id: org_id.into(),
            name: name.into(),
            properties,
        };
        let response = self
            .request(Method::POST, &create_label_url)
            .body(serde_json::to_string(&body).context(SerializingSnafu)?)
            .send()
            .await
            .context(ReqwestProcessingSnafu)?;
        match response.status() {
            StatusCode::CREATED => Ok(response
                .json::<LabelResponse>()
                .await
                .context(ReqwestProcessingSnafu)?),
            status => {
                let text = response.text().await.context(ReqwestProcessingSnafu)?;
                HttpSnafu { status, text }.fail()?
            }
        }
    }

    /// Update a Label
    pub async fn update_label(
        &self,
        name: Option<String>,
        properties: Option<HashMap<String, String>>,
        label_id: &str,
    ) -> Result<LabelResponse, RequestError> {
        let update_label_url = format!("{}/api/v2/labels/{}", &self.url, label_id);
        let body = LabelUpdate { name, properties };
        let response = self
            .request(Method::PATCH, &update_label_url)
            .body(serde_json::to_string(&body).context(SerializingSnafu)?)
            .send()
            .await
            .context(ReqwestProcessingSnafu)?;
        match response.status() {
            StatusCode::OK => Ok(response
                .json::<LabelResponse>()
                .await
                .context(ReqwestProcessingSnafu)?),
            status => {
                let text = response.text().await.context(ReqwestProcessingSnafu)?;
                HttpSnafu { status, text }.fail()?
            }
        }
    }

    /// Delete a Label
    pub async fn delete_label(&self, label_id: &str) -> Result<(), RequestError> {
        let delete_label_url = format!("{}/api/v2/labels/{}", &self.url, label_id);
        let response = self
            .request(Method::DELETE, &delete_label_url)
            .send()
            .await
            .context(ReqwestProcessingSnafu)?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await.context(ReqwestProcessingSnafu)?;
                HttpSnafu { status, text }.fail()?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    const BASE_PATH: &str = "/api/v2/labels";

    #[tokio::test]
    async fn labels() {
        let token = "some-token";

        let mock_server = mock("GET", BASE_PATH)
            .match_header("Authorization", format!("Token {}", token).as_str())
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client.labels().await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn labels_by_org() {
        let token = "some-token";
        let org_id = "some-org_id";

        let mock_server = mock("GET", format!("{}?orgID={}", BASE_PATH, org_id).as_str())
            .match_header("Authorization", format!("Token {}", token).as_str())
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client.labels_by_org(org_id).await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn find_label() {
        let token = "some-token";
        let label_id = "some-id";

        let mock_server = mock("GET", format!("{}/{}", BASE_PATH, label_id).as_str())
            .match_header("Authorization", format!("Token {}", token).as_str())
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client.find_label(label_id).await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn create_label() {
        let token = "some-token";
        let org_id = "some-org";
        let name = "some-user";
        let mut properties = HashMap::new();
        properties.insert("some-key".to_string(), "some-value".to_string());

        let mock_server = mock("POST", BASE_PATH)
            .match_header("Authorization", format!("Token {}", token).as_str())
            .match_body(
                format!(
                    r#"{{"orgID":"{}","name":"{}","properties":{{"some-key":"some-value"}}}}"#,
                    org_id, name
                )
                .as_str(),
            )
            .create();

        let client = Client::new(&mockito::server_url(), org_id, token);

        let _result = client.create_label(org_id, name, Some(properties)).await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn create_label_opt() {
        let token = "some-token";
        let org_id = "some-org_id";
        let name = "some-user";

        let mock_server = mock("POST", BASE_PATH)
            .match_header("Authorization", format!("Token {}", token).as_str())
            .match_body(format!(r#"{{"orgID":"{}","name":"{}"}}"#, org_id, name).as_str())
            .create();

        let client = Client::new(&mockito::server_url(), org_id, token);

        let _result = client.create_label(org_id, name, None).await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn update_label() {
        let token = "some-token";
        let name = "some-user";
        let label_id = "some-label_id";
        let mut properties = HashMap::new();
        properties.insert("some-key".to_string(), "some-value".to_string());

        let mock_server = mock("PATCH", format!("{}/{}", BASE_PATH, label_id).as_str())
            .match_header("Authorization", format!("Token {}", token).as_str())
            .match_body(
                format!(
                    r#"{{"name":"{}","properties":{{"some-key":"some-value"}}}}"#,
                    name
                )
                .as_str(),
            )
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client
            .update_label(Some(name.to_string()), Some(properties), label_id)
            .await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn update_label_opt() {
        let token = "some-token";
        let label_id = "some-label_id";

        let mock_server = mock("PATCH", format!("{}/{}", BASE_PATH, label_id).as_str())
            .match_header("Authorization", format!("Token {}", token).as_str())
            .match_body("{}")
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client.update_label(None, None, label_id).await;

        mock_server.assert();
    }

    #[tokio::test]
    async fn delete_label() {
        let token = "some-token";
        let label_id = "some-label_id";

        let mock_server = mock("DELETE", format!("{}/{}", BASE_PATH, label_id).as_str())
            .match_header("Authorization", format!("Token {}", token).as_str())
            .create();

        let client = Client::new(&mockito::server_url(), "", token);

        let _result = client.delete_label(label_id).await;

        mock_server.assert();
    }
}
