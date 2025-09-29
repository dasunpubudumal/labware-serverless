//! Location Types Service
//!
//! This module provides functions and types for managing location types in the application.
//! It abstracts DynamoDB operations and provides a response type for API Gateway integration.
//!
//! # Features
//! - Conditional re-export of DynamoDB implementation for testing and production
//! - API response struct for location types
//! - Function to create a location type in DynamoDB
//! - Unit tests using mock DynamoDB

use std::collections::HashMap;

use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use aws_sdk_dynamodb::types::AttributeValue;
use http::HeaderMap;
use lambda_runtime::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// DynamoDB client abstraction.
///
/// Uses the real implementation in production and a mock in tests.
#[cfg(not(test))]
pub use crate::wrappers::DynamoDBImpl as DynamoDB;
#[cfg(test)]
pub use crate::wrappers::MockDynamoDBImpl as DynamoDB;

/// API response for location type creation.
#[derive(Serialize, Deserialize)]
pub struct LocationTypeResponse {
    /// The name of the location type.
    name: String,
}

/// Creates a new location type in DynamoDB and returns an API Gateway response.
///
/// # Arguments
/// * `name` - The name of the location type to create.
/// * `client` - The DynamoDB client abstraction (real or mock).
///
/// # Returns
/// * `ApiGatewayProxyResponse` - The HTTP response for API Gateway.
pub async fn create_location_type(
    name: &str,
    client: DynamoDB,
) -> Result<ApiGatewayProxyResponse, Error> {
    let name_av = AttributeValue::S(name.to_string());

    let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();
    hashmap.insert(String::from("name"), name_av);
    let response = client.put_item("location_types", hashmap).await.unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let response = LocationTypeResponse {
        name: format!("{:?}", response.get("name")),
    };
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        body: Some(json!(response).to_string().into()),
        is_base64_encoded: false,
        headers,
    })
}

/// Unit tests for location types service.
#[cfg(test)]
mod tests {
    // References:
    // 1. https://docs.aws.amazon.com/sdk-for-rust/latest/dg/testing.html
    // 2. https://docs.aws.amazon.com/sdk-for-rust/latest/dg/testing-automock.html
    // 3. https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rustv1/examples/testing/src/wrapper.rs

    use super::*;
    use crate::wrappers::MockDynamoDBImpl;
    use mockall::predicate::eq;

    /// Tests the creation of a location type using the mock DynamoDB client.
    #[tokio::test]
    async fn test_create_location_type() {
        let mut mock = MockDynamoDBImpl::default();
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("name".to_string(), AttributeValue::S("Freezer".to_string()));
        mock.expect_put_item()
            .with(eq("location_types"), eq(map.clone()))
            .return_once(|_, _| Ok(map));
        let result = create_location_type("Freezer", mock).await.unwrap();
        assert_eq!(200, result.status_code);
    }
}
