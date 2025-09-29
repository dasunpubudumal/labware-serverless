use lambda_runtime::{run, service_fn, tracing, Error};
mod controller;
mod http_handler;
mod services;
mod wrappers;
use http_handler::handler;

/**
Official documentations
- AWS SDK https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html
- Cargo Lambda https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html
**/

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(handler)).await
}
