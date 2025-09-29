use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{Error, LambdaEvent};

/// REST Docs: https://docs.aws.amazon.com/lambda/latest/dg/rust-http-events.html
// TODO: Complete this function
pub(crate) async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some("Hello AWS Lambda HTTP request".into()),
        headers,
    };
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::apigw::ApiGatewayProxyRequest;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = ApiGatewayProxyRequest {
            resource: Some(String::from("labware-locations")),
            ..Default::default()
        };
        let event = LambdaEvent {
            payload: request,
            context: Context::default(),
        };

        let response = handler(event).await.unwrap();
        assert_eq!(response.status_code, 200);

        let body_bytes = response.body.unwrap().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();
        assert_eq!(body_string, "Hello AWS Lambda HTTP request")
    }
}
