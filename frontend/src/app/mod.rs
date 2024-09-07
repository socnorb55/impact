use yew::prelude::*;
use yew_router::prelude::*;

pub mod calculator;
pub mod home;
pub mod login;

use calculator::Calculator;
use home::Home;
use login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/calculator")]
    Calculator,
    #[at("/login")]
    Login,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Home => html! { <Home /> },
        Route::Calculator => html! { <Calculator /> },
        Route::Login => html! { <Login /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
