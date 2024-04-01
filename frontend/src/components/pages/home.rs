use std::ops::Deref;

use common::types::UserResponse;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::{
    components::{
        atoms::{
            form_title::TextTitle, spinner::Spinner, text_error::TextError, text_input::TextInput,
        },
        molecules::header::Header,
        pages::classes::{box_div_classes, main_div_classes, submit_button_classes},
        router::Route,
    },
    store::Store,
};

struct FormState {
    pub is_loading: bool,
    pub is_error: bool,
    pub message: Option<AttrValue>,
}

#[function_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let form_state = use_state(|| FormState {
        is_loading: false,
        is_error: false,
        message: None,
    });

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
            <Header />
                <div class={main_div_classes()}>
                    <div class={box_div_classes()}>
                    <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                        <div class="flex justify-center">
                            <TextTitle message={"Start playing!"} />
                        </div> 
                            <form class={"space-y-4 md:space-y-6"}>
                                <input />
                                if let Some(res) = &form_state.deref().message {
                                    if form_state.is_error {
                                        <TextError error={res.clone()}/>
                                    }
                                }

                                <div class={"flex justify-center"}>
                                    <button type={"button"} class={submit_button_classes()}>
                                        <div class={"flex justify-center"}>
                                            <span>{"Join existing session"}</span>
                                            if form_state.clone().deref().is_loading {
                                                <Spinner />
                                            }
                                        </div>
                                    </button>
                                    <button type={"button"} class={submit_button_classes()}>
                                        <div class={"flex justify-center"}>
                                            <span>{"Create new room"}</span>
                                            if form_state.clone().deref().is_loading {
                                                <Spinner />
                                            }
                                        </div>
                                    </button>
                                </div>
                            </form>
                    </div>
                    </div>
                </div>
        } else {
            <h1>{"loading..."}</h1>
        }
    }
}
