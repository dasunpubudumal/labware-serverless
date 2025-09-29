use std::collections::HashMap;

use aws_sdk_dynamodb::{client::Client, types::AttributeValue};
use lambda_runtime::Error;
#[allow(unused_imports)]
use mockall::automock;

#[allow(dead_code)]
pub struct DynamoDBImpl {
    inner: Client,
}

#[allow(dead_code)]
pub struct DynamoDBMockImpl {}

#[cfg_attr(test, automock)]
impl DynamoDBImpl {
    #[allow(dead_code)]
    pub fn new(inner: Client) -> Self {
        Self { inner }
    }

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
        Ok(response.attributes().unwrap().to_owned())
    }
}
