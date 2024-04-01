use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::StartResetRequest;
use gloo_net::http::Request;
use validator::{Validate, ValidationErrors};
use web_sys::console::log_1;
use yew::{platform::spawn_local, prelude::*};

use crate::components::{
    atoms::{
        form_title::TextTitle, logo::Logo, spinner::Spinner, text_error::TextError,
        text_input::TextInput, text_success::TextSuccess,
    },
    pages::classes::{box_div_classes, main_div_classes, submit_button_classes},
};

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<StartResetRequest>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
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

#[function_component(StartReset)]
pub fn start_reset() -> Html {
    let form: _ = use_state(|| StartResetRequest { email: "".into() });
    let form_state = use_state(|| FormState {
        is_loading: false,
        is_error: false,
        message: None,
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onsubmit = {
        let form = form.clone();
        let validation_errors = validation_errors.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match form.validate() {
                Ok(()) => {
                    let form = form.deref().clone();
                    let form_state = form_state.clone();
                    spawn_local(async move {
                        form_state.set(FormState {
                            is_error: false,
                            message: None,
                            is_loading: true,
                        });

                        let body = serde_json::to_string(&form).unwrap();
                        match Request::post("/api/auth/start-reset")
                            .header("Content-Type", "application/json")
                            .body(Some(body))
                            .send()
                            .await
                        {
                            Ok(_r) => form_state.set(FormState {
                                is_error: false,
                                message: Some(
                                    "An email has been sent. You may close this page now.".into(),
                                ),
                                is_loading: false,
                            }),
                            Err(e) => {
                                log_1(&e.to_string().into());

                                form_state.set(FormState {
                                    is_error: true,
                                    message: Some(
                                        "Something went wrong when sending request".into(),
                                    ),
                                    is_loading: false,
                                })
                            }
                        }
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
                "email" => data.email = value,
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

    let email_change = get_input_callback("email", form.clone());
    html! {
       <div class={main_div_classes()}>
           <Logo/>
           <div class={box_div_classes()}>
               <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                   <TextTitle message={"Reset password form"} />
                   <form class={"space-y-4 md:space-y-6"} {onsubmit}>
                        <TextInput label={"Email"} name={"email"} t={"email"} handle_onchange={email_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        if let Some(res) = &form_state.deref().message {
                            if form_state.is_error {
                                <TextError error={res.clone()}/>
                            } else {
                                <TextSuccess message={res.clone()}/>
                            }
                        }
                        <button type={"submit"} class={submit_button_classes()}>
                           <div class={"flex justify-center"}>
                               <span>{"Send reset link"}</span>
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
