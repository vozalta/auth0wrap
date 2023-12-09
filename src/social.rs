#![allow(non_snake_case)]

//		Modules

#[cfg(test)]
#[path = "tests/social.rs"]
mod tests;

//  Packages

use std::error::Error;
use super::auth0_client::Auth0Client;

//  Structs
///		Social
/// Data representing the social module.
///
pub struct Social {
	oauth_client: Auth0Client,
}


impl Social {
	//	  	Social
	///		
	/// Create a new Social object.
	///
	/// # Parameters
	///
	/// * `oauth_client` - The Auth0Client to use to interact with Auth0 APIs.
	///
	pub fn new(oauth_client: Auth0Client) -> Self {
		Self {
			oauth_client,
		}
	}
	
	
	//	  	Authorize
	///
	/// Authenticate a user with a social provider.
	///
	/// # Parameters
	///
	/// * `client_id` - The client ID of your Auth0 application.
	/// * `response_type` - The type of response you want from Auth0. This should be "code".
	/// * `redirect_uri` - The URL to which Auth0 will redirect the browser after authorization has been granted by the user.
	/// * `scope` - The scopes which you want to request authorization for. These must be separated by a space.
	/// * `state` - An opaque value used to prevent CSRF attacks.
	pub async fn authorize(&self, client_id: &str, response_type: &str, redirect_uri: &str, scope: &str, state: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/authorize?client_id={}&response_type={}&redirect_uri={}&scope={}&state={}",
			self.oauth_client.domain, client_id, response_type, redirect_uri, scope, state);
		
		let response = self.oauth_client.client.get(&url)
			.send()
			.await?
			.text()
			.await?;
		
		Ok(response)
	}
	
	
	//	  	GetAccessToken
	/// Get an access token from Auth0.
	///
	/// # Parameters
	///
	/// * `client_id` - The client ID of your Auth0 application.
	/// * `client_secret` - The client secret of your Auth0 application.
	/// * `audience` - The audience of your Auth0 application.
	/// * `grant_type` - The grant type for the token request. This should be "client_credentials".
	pub async fn get_access_token(&self, client_id: &str, client_secret: &str, audience: &str, grant_type: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/oauth/token", self.oauth_client.domain);
		let params = [("client_id", client_id), ("client_secret", client_secret), ("audience", audience), ("grant_type", grant_type)];
		
		let response = self.oauth_client.client.post(&url)
			.form(&params)
			.send()
			.await?
			.text()
			.await?;
		
		Ok(response)
	}
	
	//	  	GetUserInfo
	/// Get user information from Auth0.
	///
	/// # Parameters
	///
	/// * `access_token` - The access token obtained from the `get_access_token` method.
	pub async fn get_user_info(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/userinfo", self.oauth_client.domain);
		
		let response = self.oauth_client.client.get(&url)
			.bearer_auth(access_token)
			.send()
			.await?
			.text()
			.await?;
		
		Ok(response)
	}
	
	//	  	Logout
	/// Log out a user from Auth0.
	///
	/// # Parameters
	///
	/// * `client_id` - The client ID of your Auth0 application.
	/// * `return_to` - The URL to which Auth0 will redirect the browser after logout.
	pub async fn logout(&self, client_id: &str, return_to: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/v2/logout?client_id={}&returnTo={}", self.oauth_client.domain, client_id, return_to);
		
		let response = self.oauth_client.client.get(&url)
			.send()
			.await?
			.text()
			.await?;
		
		Ok(response)
	}
}

