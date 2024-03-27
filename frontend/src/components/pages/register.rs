use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::RegisterRequest;
use validator::{Validate, ValidationErrors};
use yew::prelude::*;
use yew_router::{components::Link, hooks::use_navigator};

use crate::components::{
    atoms::{form_title::TextTitle, logo::Logo, text_input::TextInput},
    pages::classes::{box_div_classes, main_div_classes, submit_button_classes},
    router::Route,
};

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterRequest>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "email" => data.email = value,
            "first_name" => data.first_name = value,
            "last_name" => data.last_name = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}
#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let form: _ = use_state(|| RegisterRequest {
        username: "".into(),
        email: "".into(),
        first_name: "".into(),
        last_name: "".into(),
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

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
                "username" => data.username = value,
                "email" => data.email = value,
                "first_name" => data.first_name = value,
                "last_name" => data.last_name = value,
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
    let email_change = get_input_callback("email", form.clone());
    let first_name_change = get_input_callback("first_name", form.clone());
    let last_name_change = get_input_callback("last_name", form.clone());
    html! {
       <div class={main_div_classes()}>
           <Logo/>
           <div class={box_div_classes()}>
               <div class={"p-6 space-y-4 md:space-y-6 sm:p-8"}>
                   <TextTitle message={"Create your account"} />
                   <form onsubmit={onsubmit} class={"space-y-4 md:space-y-6"}>
                        <TextInput label={"Username"} name={"username"} handle_onchange={username_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <TextInput label={"First name"} name={"first_name"} handle_onchange={first_name_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <TextInput label={"Last name"} name={"last_name"} handle_onchange={last_name_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <TextInput label={"Email"} name={"email"} handle_onchange={email_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                       <button type={"submit"} class={submit_button_classes()}>
                           <div class={"flex justify-center"}>
                               <span>{"Create account"}</span>
                           </div>
                       </button>
                       <p class="text-sm font-light text-gray-500 dark:text-gray-400">
                           {"Already registered? "} <Link<Route> to={Route::Login} classes={"font-medium text-primary-600 hover:underline dark:text-primary-500"}>{"Sign in"}</Link<Route>>
                       </p>
                   </form>
               </div>
           </div>
       </div>

    }
}
