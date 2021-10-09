use reqwest::Client;
use serde::Serialize;
use crate::api::response::{APIObjectResponse, APIResponse};
use crate::errors::{PxollyError, PxollyResult};

#[derive(Clone)]
pub struct APIClient {
	access_token: String,
	version: f32,
	client: Client,
}

impl APIClient {
	pub fn new(access_token: impl Into<String>, version: f32) -> Self {
		Self {
			version,
			access_token: access_token.into(),
			client: Client::new()
		}
	}

	pub fn make_url(&self, method: String) -> String {
		format!("https://api.vk.com/method/{}", method)
	}

	pub fn make_params(&self, params: impl Serialize) -> PxollyResult<serde_json::Value> {
		let mut params = serde_json::to_value(params)?;
		let ref_params = params.as_object_mut()
			.ok_or_else(|| PxollyError::None)?;
		ref_params.insert("access_token".into(), serde_json::Value::from(self.access_token.to_string()));
		ref_params.insert("v".into(), serde_json::Value::from(self.version));

		Ok(params)
	}

	pub async fn api_request(&self, method: impl Into<String>, params: impl Serialize) -> PxollyResult<APIObjectResponse> {
		let request_builder = self.client
			.post(self.make_url(method.into()))
			.form(&self.make_params(params)?);

		let response = request_builder
			.send()
			.await?
			.json::<APIResponse>()
			.await?;

		match response {
			APIResponse::Response(response) => Ok(response),
			APIResponse::Error(error) => Err(PxollyError::API(error))
		}
	}
}