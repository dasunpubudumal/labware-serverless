//! Wrappers for DynamoDB client implementations.
//!
//! This module provides an abstraction over the AWS DynamoDB client, allowing for easier mocking and testing.
//! It includes both a real implementation (`DynamoDBImpl`) and a mock implementation (`DynamoDBMockImpl`).
//!
//! The `automock` attribute from the `mockall` crate is used for conditional mocking in tests.
//!
//! # Usage
//! - Use `DynamoDBImpl` in production code for actual DynamoDB operations.
//! - In tests, the `automock` attribute enables mocking of the implementation for unit testing.

use std::collections::HashMap;

use aws_sdk_dynamodb::{client::Client, types::AttributeValue};
use lambda_runtime::Error;
#[allow(unused_imports)]
use mockall::automock;

/// Real implementation of DynamoDB client wrapper.
///
/// Wraps the AWS SDK DynamoDB client and provides methods for interacting with DynamoDB tables.
#[allow(dead_code)]
pub struct DynamoDBImpl {
    /// The inner AWS SDK DynamoDB client.
    pub(crate) inner: Client,
}

/// Mock implementation for DynamoDB client wrapper.
///
/// Used for testing purposes. Can be extended to provide mock behavior.
#[allow(dead_code)]
pub struct DynamoDBMockImpl {}

/// Implementation of DynamoDB operations.
///
/// The `automock` attribute enables mocking for tests.
#[cfg_attr(test, automock)]
impl DynamoDBImpl {
    /// Creates a new wrapper around the AWS SDK DynamoDB client.
    #[allow(dead_code)]
    pub fn new(inner: Client) -> Self {
        Self { inner }
    }

    /// Puts an item into the specified DynamoDB table.
    ///
    /// # Arguments
    /// * `table` - The name of the DynamoDB table.
    /// * `item` - The item to insert, as a map of attribute names to values.
    ///
    /// # Returns
    /// * `Result<HashMap<String, AttributeValue>, Error>` - The attributes returned by DynamoDB, or an error.
    pub async fn put_item(
        &self,
        table: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Result<HashMap<String, AttributeValue>, Error> {
        let request = self
            .inner
            .put_item()
            .table_name(table.to_string())
            .set_item(Some(item));
        let response = request.send().await?;
        println!("Response: {:?}", response);
        Ok(response.attributes().unwrap().to_owned())
    }
}
