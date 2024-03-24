use yew::prelude::*;
use yew_router::components::Link;

use crate::components::{
    atoms::{
        form_title::TextTitle, logo::Logo, password_input::PasswordInput, text_input::TextInput,
    },
    router::Route,
};

#[function_component(Login)]
pub fn login() -> Html {
    let main_div_classes = classes!(
        "flex",
        "flex-col",
        "h-screen",
        "flex-grow",
        "items-center",
        "justify-center",
        "px-6",
        "py-8",
        "mx-auto",
        "lg:py-0"
    );

    let box_div_classes = classes!(
        "w-full",
        "bg-white",
        "rounded-lg",
        "shadow",
        "dark:border",
        "md:mt-0",
        "sm:max-w-md",
        "xl:p-0",
        "dark:bg-gray-800",
        "dark:border-gray-700"
    );

    let submit_button_classes = classes!(
        "w-full",
        "text-white",
        "bg-primary-600",
        "hover:bg-primary-700",
        "focus:ring-4",
        "focus:outline-none",
        "focus:ring-primary-300",
        "font-medium",
        "rounded-lg",
        "text-sm",
        "px-5",
        "py-2.5",
        "text-center",
        "dark:bg-primary-600",
        "dark:hover:bg-primary-700",
        "dark:focus:ring-primary-800"
    );
    html! {

    <div class={main_div_classes}>
        <Logo/>
        <div class={box_div_classes}>
            <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                <TextTitle message={"Sign in your account"} />
                <form class={"space-y-4 md:space-y-6"} action="/api/auth/sign-in" method="post">
                    <TextInput label={"Username"} name={"username"} required={true} />
                    <PasswordInput label={"Password"} name={"password"} />

                    <div class={"flex items-center justify-between"}>
                        <a class={"text-sm font-medium text-primary-600 hover:underline dark:text-primary-500"}>{"Forgot password?"}</a>
                    </div>
                    <button type={"submit"} class={submit_button_classes}>
                        <div class={"flex justify-center"}>
                            <span>{"Sign in"}</span>
                        </div>
                    </button>
                </form>
                <p class={"text-sm font-light text-gray-500 dark:text-gray-400"}>
                    {"Don’t have an account yet? "} <Link<Route> to={Route::Register} classes={"font-medium text-primary-600 hover:underline dark:text-primary-500"}>{"Sign up"}</Link<Route>>
                </p>
            </div>
        </div>
    </div>
     }
}
