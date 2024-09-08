use yew::prelude::*;

#[function_component(Signup)]
pub fn signup() -> Html {
    html! {
        <div class="login">
            <form>
                <label for="email">{ "Email Address" }</label><br />
                <input id="email" type="text" name="email" /><br /><br />
                <label for="password">{ "Password" }</label><br />
                <input id="password" type="text" name="password" /><br /><br />
                <label for="confirm_password">{ "Confirm Password" }</label><br />
                <input id="confirm_password" type="text" name="confirm_password" /><br /><br />
                <label for="first_name">{ "First Name" }</label><br />
                <input id="first_name" type="text" name="first_name" /><br /><br />
                <label for="last_name">{ "Last Name" }</label><br />
                <input id="last_name" type="text" name="last_name" /><br /><br />
                <input id="login_submit" type="submit" />
            </form>
            { "Sign Up" }
            <link />
        </div>
    }
}
