#![allow(non_snake_case)]

//		Tests
use super::*;
use std::env;

use tokio;
	
#[tokio::test]
async fn test_authorize() {
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("GET", "/authorize?client_id=test_client_id&response_type=code&redirect_uri=test_redirect_uri&scope=test_scope&state=test_state")
		.with_status(200)
		.with_body("test response")
		.create();
	
	let auth0_client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let social = Social::new(auth0_client);
	
	let result = social.authorize("test_client_id", "code", "test_redirect_uri", "test_scope", "test_state").await;
	
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test response");
}

#[tokio::test]
async fn test_get_access_token() {
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("POST", "/oauth/token")
		.with_status(200)
		.with_body("test access token")
		.create();
	
	let auth0_client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let social = Social::new(auth0_client);
	
	let result = social.get_access_token("test_client_id", "test_client_secret", "test_api_url", "client_credentials").await;
	
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test access token");
}


#[tokio::test]
async fn test_get_user_info() {
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("GET", "/userinfo")
		.with_status(200)
		.with_body("test user info")
		.create();
	
	let auth0_client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let social = Social::new(auth0_client);
	
	let result = social.get_user_info("test_access_token").await;
	
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test user info");
}

#[tokio::test]
async fn test_logout() {
	let mut server = mockito::Server::new_with_port(0);
	let domain = server.url();
	
	let _m = server.mock("GET", "/v2/logout?client_id=test_client_id&returnTo=test_return_to")
		.with_status(200)
		.with_body("test logout response")
		.create();
	
	let auth0_client = Auth0Client {
		domain: domain.to_string(),
		client_id: "test_client_id".to_string(),
		client_secret: "test_client_secret".to_string(),
		audience: "test_api_url".to_string(),
		client: Client::new(),
	};
	
	let social = Social::new(auth0_client);
	
	let result = social.logout("test_client_id", "test_return_to").await;
	
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test logout response");
}