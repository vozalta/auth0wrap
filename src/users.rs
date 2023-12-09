#![allow(non_snake_case)]

//		Modules

#[cfg(test)]
#[path = "tests/users.rs"]
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
	
	//	  	get_user_profile
	///
	/// Get the user profile from the Auth0 API.
	///
	/// # Parameters
	///
	/// * `access_token` - The access token.
	///
	pub async fn get_user_profile(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
		let url = format!("{}/userinfo", self.oauth_client.domain);
		let response = self.oauth_client.client.get(&url)
			.bearer_auth(access_token)
			.send()
			.await?
			.text()
			.await?;
		Ok(response)
	}
}