use common::types::UserResponse;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::{components::Redirect, hooks::use_navigator, BrowserRouter, Switch};
use yewdux::prelude::*;

use crate::{
    components::router::{switch, Route},
    store::Store,
};

#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let location = use_location();
    use_effect(move || {
        spawn_local(async move {
            match Request::get("/api/auth/me").send().await {
                Ok(res) => {
                    if res.ok() {
                        let user = res.json::<UserResponse>().await.unwrap();
                        dispatch.reduce_mut(move |s| s.auth_user = Some(user.into()));
                    }
                }
                // network error
                Err(err) => {
                    log_1(&err.to_string().into());
                }
            };
        });
    });
    html! {

        <BrowserRouter>
            <Switch<Route> render={switch} />
            if let Some(_) = &store.auth_user {
                <Redirect<Route> to={Route::Home}></Redirect<Route>>
            } else if location.pathname != "/change-password" {
                <Redirect<Route> to={Route::Login}></Redirect<Route>>
            }
        </BrowserRouter>
    }
}
