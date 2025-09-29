use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::Error;

use crate::services::location_types::create_location_type;

pub async fn route(endpoint: &str) -> Result<ApiGatewayProxyResponse, Error> {
    match endpoint {
        "/location-types" => return Ok(ApiGatewayProxyResponse::default()),
        "/locations" => return Ok(ApiGatewayProxyResponse::default()),
        // "/labware " => return Ok(ApiGatewayProxyResponse::default()),
        _ => return Ok(ApiGatewayProxyResponse::default()),
    }
}
