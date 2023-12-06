#![allow(non_snake_case)]

//		Tests

use super::*;
use std::env;

use tokio;

#[tokio::test]
async fn test_get_user_profile() {
	//env::set_var("RUST_BACKTRACE", "1");
	let mut server = mockito::Server::new();
	let domain = server.url();
	
	let _m = server.mock("GET", "/")
		.with_status(200)
		.with_body("test user profile")
		.create();
	
	let client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "https://dev-7rd32e8tkyoho4nc.us.auth0.com/api/v2/".to_string(),
		client: Client::new(),
	};
	
	let result = client.get_user_profile("test_access_token").await;

	println!("Result: {:?}", result);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test user profile");
}
	
