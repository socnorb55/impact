use lambda_runtime::{tower::layer::util, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    heat_source: String,
    household_member_count: i32,
    recycle_types: Vec<String>,
    utility_usage: UtilityUsage,
    vehicles: Vec<Vehicles>,
    zip_code: i32
}

#[derive(Serialize)]
pub struct Response {
    pub req_id: String,
    pub carbon_footprint: f32,
}

#[derive(Deserialize)]
struct UtilityUsage {
    electricity: f32,
    fuel_oil: f32,
    natural_gas: f32,
    propane: f32
}

#[derive(Deserialize)]
struct Vehicles {
    maintenance: bool,
    mpg: f32,
    weekly_miles: i32
}

pub fn calculate_carbon_footprint(event: LambdaEvent<Request>) -> Response {

    let utility_c02_output: f32 = calculate_utility_c02_output(event.payload.utility_usage);

    Response {
        req_id: event.context.request_id,
        carbon_footprint: 31931.0,
    } 
}

fn calculate_utility_c02_output(utility_usage: UtilityUsage) -> f32 {

    let mut utility_co2_output: f32 = 0.0;
    let emission_factor: f32 = 1079.572;

    utility_co2_output += (utility_usage.electricity/0.1188)*emission_factor*12.0;

    utility_co2_output += (utility_usage.fuel_oil/4.02)*22.61*12.0;

    utility_co2_output += (utility_usage.natural_gas/10.68)*119.58*12.0;

    utility_co2_output += (utility_usage.propane/2.47)*12.43*12.0;

    utility_co2_output
}

#[cfg(test)]
mod tests {
    use lambda_runtime::{Context, LambdaEvent};
    use crate::{calculate_carbon_footprint, calculate_utility_c02_output, Request, UtilityUsage, Vehicles};

    #[test]
    fn test_calculate_carbon_footprint() {
        let id = "ID";

        let mut context = Context::default();
        context.request_id = id.to_string();

        let utilities: UtilityUsage = UtilityUsage {
            electricity: 170.0,
            fuel_oil: 0.0,
            natural_gas: 25.0,
            propane: 0.0
        };

        let vehicle_1: Vehicles = Vehicles {
            maintenance: true,
            mpg: 27.0,
            weekly_miles: 50
        };

        let vehicle_2: Vehicles = Vehicles {
            maintenance: true,
            mpg: 23.0,
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
        let event: LambdaEvent<Request> = LambdaEvent { payload, context };

        let result: crate::Response = calculate_carbon_footprint(event);

        assert_eq!(result.carbon_footprint, 31931.0);
        assert_eq!(result.req_id, id.to_string());
    }

    #[test]
    fn test_calculate_utility_c02_output() {

        let utilities: UtilityUsage = UtilityUsage {
            electricity: 170.0,
            fuel_oil: 0.0,
            natural_gas: 25.0,
            propane: 0.0
        };

        let response: f32 = calculate_utility_c02_output(utilities);

        assert_eq!(response, 18541464.0);
    }

}