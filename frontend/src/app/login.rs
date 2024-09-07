use yew::prelude::*;

#[function_component(Login)]
pub fn calculator() -> Html {
    html! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight">{ "Sign in to your account" }</h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6" action="#" method="POST">
            <div>
                <label for="email" class="block text-sm font-medium leading-6">{ "Email address" }</label>
                <div class="mt-2">
                <input id="email" name="email" type="email" autocomplete="email" required=true class="block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 ring-inset ring-eerie-black focus:ring-2 focus:ring-inset focus:ring-zomp sm:text-sm sm:leading-6" />
                </div>
            </div>

            <div>
                <div class="flex items-center justify-between">
                <label for="password" class="block text-sm font-medium leading-6">{ "Password" }</label>
                <div class="text-sm">
                    <a href="#" class="font-semibold text-wenge hover:text-zomp">{ "Forgot password?" }</a>
                </div>
                </div>
                <div class="mt-2">
                <input id="password" name="password" type="password" autocomplete="current-password" required=true class="block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 ring-inset ring-eerie-black focus:ring-2 focus:ring-inset focus:ring-zomp sm:text-sm sm:leading-6" />
                </div>
            </div>

            <div>
                <button type="submit" class="flex w-full justify-center rounded-md bg-wenge px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-zomp focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-zomp">{ "Sign in" }</button>
            </div>
            </form>

            <p class="mt-10 text-center text-sm">
            { "Not a member?" }
            <a href="#" class="font-semibold leading-6 hover:text-zomp">{ "Start a 14 day free trial" }</a>
            </p>
        </div>
        </div>
    }
}
