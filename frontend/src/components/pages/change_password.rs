use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::ChangePasswordRequest;
use validator::{Validate, ValidationErrors};
use yew::prelude::*;
use yew_router::{hooks::use_navigator};

use crate::components::{
    atoms::{form_title::TextTitle, logo::Logo, text_input::TextInput},
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

#[function_component(ChangePassword)]
pub fn change_password() -> Html {
    let form: _ = use_state(|| ChangePasswordRequest {
        password: "".into(),
        confirm_password: "".into(),
        token: "".into(),
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let form = form.clone();
        let navigator = navigator.clone();
        let validation_errors = validation_errors.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match form.validate() {
                Ok(()) => {
                    navigator.replace(&Route::Login);
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
                        <TextInput label={"Password"} name={"password"} handle_onchange={change_password} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <TextInput label={"Confirm password"} name={"confirm_password"} handle_onchange={change_confirm_password} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <button type={"submit"} class={submit_button_classes()}>
                               <span>{"Change password"}</span>
                       </button>
                   </form>
               </div>
           </div>
       </div>

    }
}
