use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {

    tracing::init_default_subscriber();

    let func = service_fn(lambda_handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

pub(crate) async fn lambda_handler(event: LambdaEvent<impact::Request>) -> Result<impact::Response, Error> {

    Ok(impact::calculate_carbon_footprint(event))
}