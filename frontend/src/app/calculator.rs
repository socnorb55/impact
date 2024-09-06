use yew::prelude::*;

#[function_component(Calculator)]
pub fn calculator() -> Html {
    html! {
        <div class="calculator">
            <form>
                <label for="zip_code">{"Current Zip Code:"}</label><br />
                <input id="zip_code" type="text" name="zip_code" /><br /><br />
                <label for="household_count">{"Number of House Hold Members:"}</label><br />
                <input id="household_count" type="text" name="household_count" /><br /><br />
                <label for="electricity_spend">{"Previous Month's Electric Bill Amount:"}</label><br />
                <input id="electricity_spend" type="text" name="electricity_spend" /><br /><br />
                <label for="fuel_oil_spend">{"Previous Month's Fuel Oil Bill Amount:"}</label><br />
                <input id="fuel_oil_spend" type="text" name="fuel_oil_spend" /><br /><br />
                <label for="natural_gas_spend">{"Previous Month's Natural Gas Bill Amount:"}</label><br />
                <input id="natural_gas_spend" type="text" name="natural_gas_spend" /><br /><br />
                <label for="propane_spend">{"Previous Month's Propane Bill Amount:"}</label><br />
                <input id="propane_spend" type="text" name="propane_spend" /><br /><br />
                {"Please Select the Materials Your Household Recycles:"}<br />
                <input id="recycling_aluminum" type="checkbox" name="recycling_aluminum" />
                <label for="recycling_aluminum">{"Aluminum and Steel Cans"}</label><br />
                <input id="recycling_plastic" type="checkbox" name="recycling_plastic" />
                <label for="recycling_plastic">{"Plastic"}</label><br />
                <input id="recycling_glass" type="checkbox" name="recycling_glass" />
                <label for="recycling_glass">{"Glass"}</label><br />
                <input id="recycling_newspaper" type="checkbox" name="recycling_newspaper" />
                <label for="recycling_newspaper">{"Newspaper"}</label><br />
                <input id="recycling_magazine" type="checkbox" name="recycling_magazine" />
                <label for="recycling_magazine">{"Magazines"}</label><br /><br />
                {"Do You Preform Regular Maintenance On Your Car:"}<br />
                <input id="vehicle_maintenance_yes" type="radio" name="vehicle_maintenance" />
                <label for="vehicle_maintenance_yes">{"Yes"}</label><br />
                <input id="vehicle_maintenance_no" type="radio" name="vehicle_maintenance" />
                <label for="vehicle_maintenance_no">{"No"}</label><br /><br />
                <label for="vehicle_mpg">{"Vehicle's Average Miles Per Gallon:"}</label><br />
                <input id="vehicle_mpg" type="text" name="vehicle_mpg" /><br /><br />
                <label for="vehicle_miles">{"Vehicle's Weekly Miles:"}</label><br />
                <input id="vehicle_miles" type="text" name="vehicle_miles" /><br /><br />
                <input id="calculator_submit" type="submit" />
            </form>
        </div>
    }
}