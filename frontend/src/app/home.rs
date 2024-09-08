use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="hero">
                <h1>{ "What is your impact on the world?" }</h1>
                <h2>{ "Calculate your CO2 output to find out" }</h2>
            </div>
            <div class="button">
                <button>{ "Calculate your output ->" }</button>
            </div>
            <div class="nav-bar">
                <button>{ "Logo" }</button>
                <button>{ "Home" }</button>
                <button>{ "Calculator" }</button>
                <button>{ "Trends" }</button>
                <button>{ "Offset" }</button>
                <button>{ "Login/Sign up" }</button>
            </div>
            <div class="about">
                <p>
                    { "According to ____ humans produce ____ of CO2 in 202_. This is a __% increase since ____.
                    As humans we are continuously contributing to the continued rise in CO2 output which directly impacts rising sea levels, higher global average temperatures, and the rapid melting of glacial ice.
                    The first step to reversing our contributions to this growing problem is understanding the impact of our actions on the planet.
                    ___ is designed to do just that.
                    Calculate your CO2 output, see trends over time, learn how to reduce your impact, and even offset your remaining CO2 output." }
                </p>
                <button>{ "Sign Up" }</button>
                <button>{ "Trend Explorer" }</button>
                <button>{ "Offsetting Information" }</button>
            </div>
        </>
    }
}
