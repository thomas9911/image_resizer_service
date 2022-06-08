use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::Region;
use aws_sdk_s3::{Credentials, Endpoint};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Derivative, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
#[derivative(Default)]
pub struct S3Config {
    #[derivative(Default(value = "\"us-east-1\".to_string()"))]
    region: String,
    #[derivative(Default(value = "\"http://127.0.0.1:9000\".to_string()"))]
    endpoint: String,
    #[derivative(Default(value = "\"test_key_id\".to_string()"))]
    access_key: String,
    #[derivative(Default(value = "\"secret_access_key\".to_string()"))]
    secret_access_key: String,
}

impl S3Config {
    pub fn endpoint(&self) -> http::Uri {
        if let Ok(endpoint) = self.endpoint.parse() {
            endpoint
        } else {
            "http://localhost:9000".parse().expect("valid URI")
        }
    }

    pub fn region(&self) -> &str {
        self.region.as_ref()
    }

    pub fn access_key(&self) -> &str {
        self.access_key.as_ref()
    }

    pub fn secret_access_key(&self) -> &str {
        self.secret_access_key.as_ref()
    }

    /// Build `aws_sdk_s3::Client` from config
    pub async fn to_client(self) -> Client {
        let region_provider = RegionProviderChain::first_try(Region::new(self.region.to_string()))
            .or_else("us-east-1");
        let creds = Credentials::new(&self.access_key, &self.secret_access_key, None, None, "s3");
        let endpoint = Endpoint::immutable(self.endpoint());

        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(creds)
            .endpoint_resolver(endpoint)
            .load()
            .await;
        Client::new(&config)
    }

    ///
    /// # Errors
    ///
    /// Returns error when invalid json and missing the required fields
    ///
    pub fn from_json(data: &[u8]) -> serde_json::Result<S3Config> {
        serde_json::from_slice(data)
    }
}

#[test]
fn s3_config_default_test() {
    let cfg = S3Config::default();

    let expected = http::Uri::from_static("http://127.0.0.1:9000");

    assert_eq!(cfg.endpoint(), expected);
    assert_eq!(cfg.region(), "us-east-1");
    assert_eq!(cfg.access_key(), "test_key_id");
    assert_eq!(cfg.secret_access_key(), "secret_access_key");
}

#[test]
fn s3_config_updated_endpoint_test() {
    let cfg = S3Config {
        endpoint: String::from("https://assets.example.com"),
        region: String::new(),
        access_key: String::new(),
        secret_access_key: String::new(),
    };

    let expected = http::Uri::from_static("https://assets.example.com");

    assert_eq!(cfg.endpoint(), expected);
    assert_eq!(cfg.region(), "");
    assert_eq!(cfg.access_key(), "");
    assert_eq!(cfg.secret_access_key(), "");
}

#[test]
fn s3_config_from_json_test() {
    let cfg = S3Config::from_json(
        br#"{
        "region": "myregion",
        "endpoint": "https://assets.example.com",
        "access_key": "access",
        "secret_access_key": ""
    }"#,
    )
    .unwrap();

    let expected = http::Uri::from_static("https://assets.example.com");

    assert_eq!(cfg.endpoint(), expected);
    assert_eq!(cfg.region(), "myregion");
    assert_eq!(cfg.access_key(), "access");
    assert_eq!(cfg.secret_access_key(), "");
}

#[test]
fn s3_config_from_empty_json_test() {
    let cfg = S3Config::from_json(br#"{ }"#);

    assert!(cfg.is_err());
}
