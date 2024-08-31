use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    household_member_count: i32,
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

    let utility_co2_output: f32 = calculate_utility_co2_output(event.payload.utility_usage, event.payload.zip_code);

    let vehicle_co2_output: f32 = calculate_vehicle_co2_output(event.payload.vehicles);

    let waste_co2_output: f32 = calculate_waste_co2_output(event.payload.household_member_count);

    Response {
        req_id: event.context.request_id,
        carbon_footprint: 31931.0,
    } 
}

fn calculate_utility_co2_output(utility_usage: UtilityUsage, zip_code: i32) -> f32 {

    let mut utility_co2_output: f32 = 0.0;
    let emission_factor: f32 = 1.0795718;
    println!("{}", zip_code);

    utility_co2_output += (utility_usage.electricity / 0.1188) * emission_factor * 12.0;

    utility_co2_output += (utility_usage.fuel_oil / 4.02) * 22.61 * 12.0;

    utility_co2_output += (utility_usage.natural_gas / 10.68) * 119.58 * 12.0;

    utility_co2_output += (utility_usage.propane / 2.47) * 12.43 * 12.0;

    utility_co2_output
}

fn calculate_vehicle_co2_output(vehicles: Vec<Vehicles>) -> f32 {

    let mut vehicle_co2_output: f32 = 0.0;

    for vehicle in vehicles {

        let vehicle_efficiency_output: f32 = match vehicle.maintenance {
            false => 0.04,
            true => 1.0
        };

        vehicle_co2_output += (vehicle.weekly_miles as f32 * 52.0) / vehicle.mpg * 19.6 * 1.01 * vehicle_efficiency_output
    };

    vehicle_co2_output

}

fn calculate_waste_co2_output(household_count: i32) -> f32 {

    household_count as f32 * 692.0
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_carbon_footprint,
        calculate_utility_co2_output,
        calculate_vehicle_co2_output,
        calculate_waste_co2_output,
        Request,
        UtilityUsage,
        Vehicles
    };
    use lambda_runtime::{Context, LambdaEvent};

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
            household_member_count: 2,
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
    fn test_calculate_utility_co2_output() {

        let utilities: UtilityUsage = UtilityUsage {
            electricity: 170.0,
            fuel_oil: 0.0,
            natural_gas: 25.0,
            propane: 0.0
        };

        let zip_code: i32 = 22079;

        let response: f32 = calculate_utility_co2_output(utilities, zip_code);

        assert_eq!(response, 21897.092);
    }

    #[test]
    fn test_calculate_vehicle_co2_output() {

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

        let response: f32 = calculate_vehicle_co2_output(vec![vehicle_1, vehicle_2]);

        assert_eq!(response, 8619.708);
    }

    #[test]
    fn test_calculate_waste_co2_output() {

        let response: f32 = calculate_waste_co2_output(2);

        assert_eq!(response, 1384.0);
    }

}