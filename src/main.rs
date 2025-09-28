use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
use http_handler::function_handler;

/**
Official documentations
- AWS SDK https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html
- Cargo Lambda https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html
**/

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
