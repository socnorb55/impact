use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UtilityUsage {
    electricity: i32,
    fuel_oil: i32,
    natural_gas: i32,
    propane: i32
}

#[derive(Deserialize)]
struct Vehicles {
    maintenance: bool,
    mph: i32,
    weekly_miles: i32
}

#[derive(Deserialize)]
struct Request {
    heat_source: String,
    household_member_count: i32,
    recycle_types: Vec<String>,
    utility_usage: UtilityUsage,
    vehicles: Vec<Vehicles>,
    zip_code: i32
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    carbon_footprint: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    tracing::init_default_subscriber();

    let func = service_fn(lambda_handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

pub(crate) async fn lambda_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {

    let resp = Response {
        req_id: event.context.request_id,
        carbon_footprint: 31931,
    };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use crate::{lambda_handler, Request, UtilityUsage, Vehicles};
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn response_is_good_for_simple_input() {
        let id = "ID";

        let mut context = Context::default();
        context.request_id = id.to_string();

        let utilities: UtilityUsage = UtilityUsage {
            electricity: 170,
            fuel_oil: 0,
            natural_gas: 25,
            propane: 0
        };

        let vehicle_1: Vehicles = Vehicles {
            maintenance: true,
            mph: 27,
            weekly_miles: 50
        };

        let vehicle_2: Vehicles = Vehicles {
            maintenance: true,
            mph: 23,
            weekly_miles: 150
        };

        let payload = Request {
            heat_source: String::from("natural_gas"),
            household_member_count: 2,
            recycle_types: vec![],
            utility_usage: utilities,
            vehicles: vec![vehicle_1, vehicle_2],
            zip_code: 22079
        };
        let event = LambdaEvent { payload, context };

        let result = lambda_handler(event).await.unwrap();

        assert_eq!(result.carbon_footprint, 31931);
        assert_eq!(result.req_id, id.to_string());
    }
}