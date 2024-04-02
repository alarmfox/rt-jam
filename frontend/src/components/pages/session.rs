use common::types::UserResponse;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_clipboard;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{components::{molecules::header::Header, organisms::client::Client, router::Route}, store::{Store, User}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
}

#[function_component(Session)]
pub fn session(Props { id }: &Props) -> Html {
    let clipboard = use_clipboard();
    let to_clipboard = {
        let id = id.clone();
        Callback::from(move |_: MouseEvent| {
            clipboard.write_text(id.to_string());
        })
    };
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();
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

    let user = store.auth_user.clone();
    html! {
        <>
            if let Some(user) = user {
                <Header />
                <div class="flex flex-row px-4 py-2">
                    <h1>{"Session id: "}</h1>
                    <div onclick={to_clipboard} class="block text-lg bg-gray-600 cursor-pointer ">
                        {id}
                        <i class="ml-1 fa-clipboard fa-solid"></i>
                    </div>
                    <h1>{". Share it with your friends!"}</h1>
                </div>
                <div class="flex justify-center">
                    <Client username={user.username} id={id.clone().to_string()}/>
                </div>
            }
        </>
    }
}
