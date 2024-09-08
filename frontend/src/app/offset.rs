use yew::prelude::*;

#[function_component(Offset)]
pub fn offset() -> Html {
    html! {
        <>
            <div class="how_it_works">
                <h1>{ "How It Works" }</h1>
                <p>{
                    "Every effort towards reducing your CO2 output is beneficial but the reality of the world is that there reducing your emissions footprint completely is not possible.
                    That is why we have partnered with ____ to help offset your remain CO2 output.
                    Through your donation, ____ will plant trees, which actively absorb carbon from the atmosphere."
                }</p>
            </div>
            <div class="donation_tiers">
                <div class="output_donation">
                    { "Output Offset Donation" }
                </div>
                <div class="custom_donation">
                    { "Custom Offset Donation" }
                </div>
            </div>
        </>
    }
}
