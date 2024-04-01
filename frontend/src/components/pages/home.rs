use std::{cell::RefCell, ops::Deref, rc::Rc};

use chrono::naive::serde;
use common::types::{CreateRoomRequest, UserResponse};
use gloo_net::http::Request;
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
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

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<CreateRoomRequest>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

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
    let form = use_state(|| CreateRoomRequest { name: "".into() });

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let join_session = {
        let form = form.clone();
        let validation_errors = validation_errors.clone();
        let form_state = form_state.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match form.validate() {
                Ok(()) => {
                    let form = form.deref().clone();
                    let navigator = navigator.clone();
                    let form_state = form_state.clone();
                    spawn_local(async move {
                        form_state.set(FormState {
                            is_error: false,
                            message: None,
                            is_loading: true,
                        });
                        let body = serde_json::to_string(&form).unwrap();
                        match Request::post("/rooms")
                            .header("Content-Type", "application/json")
                            .body(Some(body))
                            .send()
                            .await
                        {
                            Ok(res) => {
                                if res.ok() {
                                    navigator.replace(&Route::Home);
                                    form_state.set(FormState {
                                        is_error: false,
                                        message: None,
                                        is_loading: false,
                                    });
                                } else {
                                    form_state.set(FormState {
                                        is_error: true,
                                        message: Some("Cannot create room".into()),
                                        is_loading: false,
                                    });
                                }
                            }
                            // network error
                            Err(err) => {
                                log_1(&err.to_string().into());
                            }
                        };
                    });
                }
                Err(e) => {
                    validation_errors.set(Rc::new(RefCell::new(e)));
                }
            }
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
    let onblur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "name" => data.name = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name, error.clone());
                        }
                    }
                }
            }
        })
    };

    let name_change = get_input_callback("name", form.clone());
    html! {
        if let Some(user) = user {
            <Header />
                <div class={main_div_classes()}>
                    <div class={box_div_classes()}>
                    <div class={"p-2 space-y-4 md:space-y-6 sm:p-8"}>
                        <div class="flex justify-center">
                            <TextTitle message={"Start playing!"} />
                        </div>
                                <TextInput label={"Session name"} name={"name"} handle_onchange={name_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                                if let Some(res) = &form_state.deref().message {
                                    if form_state.is_error {
                                        <TextError error={res.clone()}/>
                                    }
                                }

                                    <button  type={"button"} class={submit_button_classes()}>
                                        <div class={"flex justify-center"}>
                                            <span>{"Create new session"}</span>
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
                                            <span>{"Join existing session"}</span>
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
