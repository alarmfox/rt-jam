use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::ChangePasswordRequest;
use gloo_net::http::Request;
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, UrlSearchParams};
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::components::{
    atoms::{form_title::TextTitle, logo::Logo, spinner::Spinner, text_error::TextError, text_input::TextInput, text_success::TextSuccess},
    pages::classes::{box_div_classes, main_div_classes, submit_button_classes},
    router::Route,
};

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<ChangePasswordRequest>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "password" => data.password = value,
            "confirm_password" => data.confirm_password = value,
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

#[function_component(ChangePassword)]
pub fn change_password() -> Html {
    let form: _ = use_state(|| ChangePasswordRequest {
        password: "".into(),
        confirm_password: "".into(),
        token: "".into(),
    });
    let form_state = use_state(|| FormState {
        is_loading: false,
        is_error: false,
        message: None,
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();
    let route = use_location().unwrap();

    let onsubmit = {
        let form = form.clone();
        let navigator = navigator.clone();
        let validation_errors = validation_errors.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let query = route.query_str();
            let url_search_params = UrlSearchParams::new_with_str(query).unwrap();
            let token = url_search_params.get("token").unwrap_or("".into());
            let navigator = navigator.clone();
            let form_state = form_state.clone();
            let mut form = form.deref().clone();
            form.token = token;

            match form.validate() {
                Ok(()) => spawn_local(async move {
                    let body = serde_json::to_string(&form).unwrap();
                    form_state.set(FormState {
                        is_error: false,
                        is_loading: true,
                        message: None,
                    });
                    match Request::post("/api/auth/change-password")
                        .header("Content-Type", "application/json")
                        .body(Some(body))
                        .send()
                        .await
                    {
                        Ok(_) => {
                            form_state.set(FormState {
                                is_error: false,
                                is_loading: false,
                                message: None,
                            });
                            navigator.replace(&Route::Login);
                        }
                        Err(e) => {
                            form_state.set(FormState {
                                is_error: true,
                                is_loading: false,
                                message: Some("Something went wrong".into()),
                            });
                            log_1(&e.to_string().into());
                        }
                    }
                }),
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
                "password" => data.password = value,
                "confirm_password" => data.confirm_password = value,
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
    let change_password = get_input_callback("password", form.clone());
    let change_confirm_password = get_input_callback("confirm_password", form.clone());
    html! {
       <div class={main_div_classes()}>
           <Logo/>
           <div class={box_div_classes()}>
               <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                   <TextTitle message={"Set up password"} />
                   <form class={"space-y-4 md:space-y-6"} onsubmit={onsubmit}>
                        <TextInput label={"Password"} t="password" name={"password"} handle_onchange={change_password} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <TextInput label={"Confirm password"} t="password" name={"confirm_password"} handle_onchange={change_confirm_password} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        if let Some(res) = &form_state.deref().message {
                            if form_state.is_error {
                                <TextError error={res.clone()}/>
                            } else {
                                <TextSuccess message={res.clone()}/>
                            }
                        }
                        <button type={"submit"} class={submit_button_classes()}>
                           <div class={"flex justify-center"}>
                               <span>{"Change password"}</span>
                                if form_state.clone().deref().is_loading {
                                    <Spinner />
                                }
                           </div>
                       </button>
                   </form>
               </div>
           </div>
       </div>

    }
}
