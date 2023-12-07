#![allow(non_snake_case)]

//		Modules

#[cfg(test)]
#[path = "tests/social.rs"]
mod tests;

//  Packages
use reqwest::Client;
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
	

}