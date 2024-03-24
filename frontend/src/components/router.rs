use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::pages::{login::Login, not_found::NotFound, register::Register}, layout::Layout};

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

    #[at("/not-found")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Hello world" } </h1>
    }
}

#[function_component(ChangePassword)]
fn change_passsword() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Change password" } </h1>
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Layout>
                <h1>{ "Home" }</h1>
            </Layout>
        },
        Route::Login => html! {
            <Layout>
                <Login/>
            </Layout>
        },
        Route::Register=> html! {
            <Layout>
                <Register/>
            </Layout>
        },
        Route::ChangePassword => html! {
            <Layout>
                <ChangePassword/>
            </Layout>
        },
        Route::NotFound => {
            html! {
                <Layout>
                    <NotFound />
                </Layout>
            }
        }
    }
}
