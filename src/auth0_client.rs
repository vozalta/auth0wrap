#![allow(non_snake_case)]

//			Modules

use reqwest::Client;

//  Structs

///		Auth0Client
///  Data representing the Auth0 client.
///
///  This will be used to interact with the Auth0 APIs. This struct will be 
///  used by other Auth0-related modules.
pub struct Auth0Client {
	pub domain: String,
	pub client_id: String,
	pub client_secret: String,
	pub audience: String,
	pub client: Client,
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
}