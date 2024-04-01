use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::{CreateRoomRequest, UserResponse};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::{
    components::{
        atoms::{
            class::{label_classes, text_input_classes},
            form_title::TextTitle,
            spinner::Spinner,
            text_error::TextError,
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
    let session_id = use_state(|| "".to_string());
    let navigator = use_navigator().unwrap();
    let form_state = use_state(|| FormState {
        is_loading: false,
        is_error: false,
        message: None,
    });

    let create_session = {
        let form_state = form_state.clone();
        let navigator = navigator.clone();
        let session_id = session_id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let form_state = form_state.clone();
            let session_id = session_id.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                form_state.set(FormState {
                    is_error: false,
                    is_loading: true,
                    message: None,
                });
                let url = format!("/api/rooms/{}", session_id.clone().deref());
                match Request::get(url.as_ref()).send().await {
                    Ok(res) => {
                        if res.ok() {
                            navigator.push(&Route::Session {
                                id: session_id.to_string(),
                            });
                        }
                        form_state.set(FormState {
                            is_loading: false,
                            is_error: true,
                            message: Some("Room does not exists".into()),
                        })
                    }
                    Err(e) => {
                        log_1(&e.to_string().into());
                        form_state.set(FormState {
                            is_loading: false,
                            is_error: true,
                            message: Some("Something went wrong".into()),
                        })
                    }
                }
            });
        })
    };
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
    let on_change = {
        let session_id = session_id.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            session_id.set(value);
        })
    };
    html! {
        if let Some(_user) = user {
            <Header />
                <div class={main_div_classes()}>
                    <div class={box_div_classes()}>
                    <div class="p-2 space-y-4 md:space-y-6 sm:p-8">
                        <div class="flex justify-center">
                            <TextTitle message={"Start playing!"} />
                        </div>
                            <div>
                                <label for={"id"} class={label_classes()}>{"Session id"}</label>
                                <input required={true} name={"Session name"} id={"id"} class={text_input_classes()} placeholder={"Session id"}
                                    onchange={on_change}
                                />
                            </div>
                            if let Some(res) = &form_state.deref().message {
                                if form_state.is_error {
                                    <TextError error={res.clone()}/>
                                }
                            }

                            <button onclick={create_session} type={"button"} class={submit_button_classes()}>
                                <div class={"flex justify-center"}>
                                    <span>{"Join existing session"}</span>
                                    if form_state.clone().deref().is_loading {
                                        <Spinner />
                                    }
                                </div>
                            </button>
                            <div class={"flex justify-center"}>
                                <h1>{"or"}</h1>
                            </div>
                            <button type={"button"} class={submit_button_classes()}>
                                <div class={"flex justify-center"}>
                                    <span>{"Create new session"}</span>
                                    if form_state.clone().deref().is_loading {
                                        <Spinner />
                                    }
                                </div>
                                </button>
                        </div>
                    </div>
                </div>
        } else {
            <h1>{"loading..."}</h1>
        }
    }
}
