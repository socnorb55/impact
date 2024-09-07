use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <header>
            <nav class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8" aria-label="Global">
                <div class="flex lg:flex-1">
                    <Link<Route> to={Route::Home} classes="text-emerald-800 underline" >
                        <img class="w-10 h-10" src="logo.svg" alt="Home" />
                    </Link<Route>>
                </div>
                <div class="flex lg:flex-1">
                    <Link<Route> to={Route::Home} classes="text-emerald-800 underline" >{ "Home" }</Link<Route>>
                </div>
                <div class="flex lg:flex-1">
                    <Link<Route> to={Route::Calculator} classes="text-emerald-800 underline">{ "Calculator" }</Link<Route>>
                </div>
            </nav>
        </header>
    }
}
