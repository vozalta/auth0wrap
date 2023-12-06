#![allow(non_snake_case)]

//		Modules
#[cfg(test)]
#[path = "tests/social.rs"]
mod tests;

//  Packages
use reqwest::Client;
use std::error::Error;



//  Structs

///		Auth0Client
///  Data representing the Auth0 client.
///
///  This will be used to interact with the Auth0 APIs.
pub struct Auth0Client {
	domain: String,
	client_id: String,
	client_secret: String,
	audience: String,
	client: Client,
}

impl Auth0Client {
	//	  	Auth0Client
	///
	/// Create a new Auth0Client.
	///
	/// # Parameters
	///
	/// * `domain` - The Auth0 domain.
	/// * `client_id` - The Auth0 client ID.
	/// * `client_secret` - The Auth0 client secret.
	/// * `audience` - The Auth0 audience.
	///
	pub fn new(domain: String, client_id: String, client_secret: String, audience: String) -> Self {
		Self {
			domain,
			client_id,
			client_secret,
			audience,
			client: Client::new(),
		}
	}
	
	//	  	get_user_profile
	///
	/// Get the user profile from the Auth0 API.
	///
	/// # Parameters
	///
	/// * `access_token` - The access token.
	///
	pub async fn get_user_profile(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/userinfo", self.domain);
		let response = self.client.get(&url)
			.bearer_auth(access_token)
			.send()
			.await?
			.text()
			.await?;
		Ok(response)
	}
}