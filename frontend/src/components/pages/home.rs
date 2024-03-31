use common::types::UserResponse;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::{components::{organisms::session::Session, router::Route}, store::{Store}};

#[function_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    
    use_effect(move || {
        spawn_local(async move {
            match Request::get("/api/auth/me").send().await {
                Ok(res) => {
                    if res.ok() {
                        let user = res.json::<UserResponse>().await.unwrap();
                        dispatch.reduce_mut(move |s| s.auth_user = Some(user.into()));
                    } else {
                        navigator.replace(&Route::Login);
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
        if let Some(user) = user {
            <h1>{user.username}</h1>
        } else {
            <h1>{"loading..."}</h1>
        }
    }
}
