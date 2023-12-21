#![allow(non_snake_case)]

//		Tests
use super::*;
use std::env;
use reqwest::Client;

use tokio;

#[tokio::test]
async fn test_get_user_profile() {
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("GET", "/userinfo")
		.with_status(200)
		.with_body("test user profile")
		.create();
	
	let auth0_client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let social = Social::new(auth0_client);
	let result = social.get_user_profile("test_access_token").await;
	
	println!("Result: {:?}", result);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test user profile");
}
	
