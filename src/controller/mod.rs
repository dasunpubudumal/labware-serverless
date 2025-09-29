use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{services::location_types::create_location_type, wrappers::DynamoDBImpl};

#[derive(Serialize, Deserialize)]
struct LocationTypeRequest {
    pub name: String,
}

pub async fn route(endpoint: &str, request_body: String) -> Result<ApiGatewayProxyResponse, Error> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let client = aws_sdk_dynamodb::Client::from_conf(config);
    // TODO: Pass name and client into create_location_type function.
    match endpoint {
        "/location-types" => {
            println!("Routed to location types.");
            println!("Req Body: {:?}", request_body);
            let location_type_request: LocationTypeRequest =
                serde_json::from_str(&request_body).unwrap();
            let name = location_type_request.name;
            println!("Name: {:?}", name);
            // Ignore rust-analyzer warning here.
            Ok(create_location_type(&name, DynamoDBImpl { inner: client })
                .await
                .unwrap())
        }
        "/locations" => Ok(ApiGatewayProxyResponse::default()),
        // "/labware " => return Ok(ApiGatewayProxyResponse::default()),
        _ => Ok(ApiGatewayProxyResponse::default()),
    }
}
