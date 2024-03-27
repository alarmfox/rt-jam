use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::{
    components::Redirect,
    BrowserRouter, Switch,
};
use yewdux::prelude::*;

use crate::{
    components::router::{switch, Route},
    store::Store,
};

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let location = use_location();
    html! {

        <BrowserRouter>
            <Switch<Route> render={switch} />
            if let Some(user) = &store.auth_user {
            } else if location.pathname != "/change-password" {
                    <Redirect<Route> to={Route::Login}></Redirect<Route>>
            }
        </BrowserRouter>
    }
}
