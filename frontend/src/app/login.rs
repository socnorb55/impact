use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="login">
            <form>
                <label for="email">{ "Email Address" }</label><br />
                <input id="email" type="text" name="email" /><br /><br />
                <label for="password">{ "Password" }</label><br />
                <input id="password" type="text" name="password" /><br /><br />
                <input id="login_submit" type="submit" />
            </form>
        </div>
    }
}
