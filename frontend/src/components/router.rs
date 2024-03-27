use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::pages::{login::Login, not_found::NotFound, register::Register},
};

use super::{layouts::simple::SimpleLayout, pages::{change_password::ChangePassword, reset_password::StartReset}};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/register")]
    Register,

    #[at("/change-password")]
    ChangePassword,

    #[at("/start-reset")]
    StartReset,

    #[at("/not-found")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <h1 class="bg-purple-600"> { "Hello world" } </h1>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <SimpleLayout>
                <h1>{ "Home" }</h1>
            </SimpleLayout>
        },
        Route::Login => html! {
            <SimpleLayout>
                <Login/>
            </SimpleLayout>
        },
        Route::Register => html! {
            <SimpleLayout>
                <Register/>
            </SimpleLayout>
        },
        Route::ChangePassword => html! {
            <SimpleLayout>
                <ChangePassword/>
            </SimpleLayout>
        },
        Route::StartReset => html! {
            <SimpleLayout>
                <StartReset/>
            </SimpleLayout>
        },
        Route::NotFound => {
            html! {
                <SimpleLayout>
                    <NotFound />
                </SimpleLayout>
            }
        }
    }
}
