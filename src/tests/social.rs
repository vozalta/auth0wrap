#![allow(non_snake_case)]

//		Tests

use super::*;
use std::env;

use tokio;

#[tokio::test]
async fn test_get_user_profile() {
	//env::set_var("RUST_BACKTRACE", "1");
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("GET", "/userinfo")
		.with_status(200)
		.with_body("test user profile")
		.create();
	
	let client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let result = client.get_user_profile("test_access_token").await;

	println!("Result: {:?}", result);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test user profile");
}
	
