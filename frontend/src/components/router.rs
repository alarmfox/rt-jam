use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{components::pages::{home::Home, login::Login, register::Register}, store::Store};

use super::{
    layouts::simple::SimpleLayout,
    pages::{change_password::ChangePassword, reset_password::StartReset},
};

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

    #[not_found]
    #[at("/not-found")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <SimpleLayout>
                    <Home />
                </SimpleLayout>
            }
        }
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
                    <h1>{"Not found"}</h1>
                </SimpleLayout>
            }
        }
    }
}
