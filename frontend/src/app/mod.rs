use yew::prelude::*;
use yew_router::prelude::*;

pub mod calculator;
pub mod home;
pub mod login;
pub mod not_found;
pub mod offset;
pub mod signup;
pub mod trends;

use calculator::Calculator;
use home::Home;
use login::Login;
use not_found::NotFound;
use offset::Offset;
use signup::Signup;
use trends::Trends;

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
    #[at("/offset")]
    Offset,
    #[at("/signup")]
    Signup,
    #[at("/trends")]
    Trends,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Calculator => html! { <Calculator /> },
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Offset => html! { <Offset /> },
        Route::Signup => html! { <Signup /> },
        Route::Trends => html! { <Trends /> },
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
