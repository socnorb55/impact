use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="404">
            <h1>{ "404 Not Found" }</h1>
        </div>
    }
}
