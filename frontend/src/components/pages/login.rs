use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::{LoginRequest, UserResponse};
use gloo_net::http::Request;
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::{components::Link, hooks::use_navigator};
use yewdux::use_store;

use crate::{
    components::{
        atoms::{form_title::TextTitle, logo::Logo, text_error::TextError, text_input::TextInput},
        pages::classes::{box_div_classes, main_div_classes, submit_button_classes},
        router::Route,
    },
    store::Store,
};

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginRequest>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(Login)]
pub fn login() -> Html {
    let (_, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let form = use_state(|| LoginRequest {
        username: "".into(),
        password: "".into(),
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let error_message = use_state(|| None::<AttrValue>);

    let onsubmit = {
        let form = form.clone();
        let validation_errors = validation_errors.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match form.validate() {
                Ok(()) => {
                    let form = form.deref().clone();
                    let dispatch = dispatch.clone();
                    let navigator = navigator.clone();
                    let error_message = error_message.clone();
                    spawn_local(async move {
                        let body = serde_json::to_string(&form).unwrap();
                        match Request::post("/api/auth/sign-in")
                            .header("Content-Type", "application/json")
                            .body(Some(body))
                            .send()
                            .await
                        {
                            Ok(res) => {
                                if res.ok() {
                                    navigator.replace(&Route::Home);
                                } else {
                                    error_message.set(Some("Invalid credentials".into()));
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
    let onblur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "username" => data.username = value,
                "password" => data.password = value,
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

    let username_change = get_input_callback("username", form.clone());
    let password_change = get_input_callback("password", form.clone());
    html! {
    <div class={main_div_classes()}>
        <Logo/>
        <div class={box_div_classes()}>
            <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                <TextTitle message={"Sign in your account"} />
                <form onsubmit={onsubmit} class={"space-y-4 md:space-y-6"}>
                    <TextInput label={"Username"} name={"username"} handle_onchange={username_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                    <TextInput t={"password"} label={"Password"} name={"password"} handle_onchange={password_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                    <div class={"flex items-center justify-between"}>
                        <Link<Route> to={Route::StartReset} classes={"text-sm font-medium text-primary-600 hover:underline dark:text-primary-500"}>{"Forgot password?"}</Link<Route>>
                    </div>
                    <TextError error={error_message.deref().clone()} />
                    <button type={"submit"} class={submit_button_classes()}>
                        <div class={"flex justify-center"}>
                            <span>{"Sign in"}</span>
                        </div>
                    </button>
                </form>
                <p class={"text-sm font-light text-gray-500 dark:text-gray-400"}>
                    {"Don’t have an account yet? "} <Link<Route> to={Route::Register} classes={"font-medium text-primary-600 hover:underline dark:text-primary-500"}>{"Sign up"}</Link<Route>>
                </p>
            </div>
        </div>
    </div>
     }
}
