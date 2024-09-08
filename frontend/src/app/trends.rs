use yew::prelude::*;

#[function_component(Trends)]
pub fn trends() -> Html {
    html! {
        <>
            <div class="weekly_average total">
            { "Weekly Total Average" }
            </div>
            <div class="weekly_average without_mitigation">
            { "Weekly Average Without Mitigation" }
            </div>
            <div class="area_average total">
            { "Local Area Total Average" }
            </div>
            <div class="national_average total">
            { "National Total Average" }
            </div>
        </>
    }
}
