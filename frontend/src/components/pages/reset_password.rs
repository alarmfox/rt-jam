use std::{cell::RefCell, ops::Deref, rc::Rc};

use common::types::StartResetRequest;
use validator::{Validate, ValidationErrors};
use yew::prelude::*;

use crate::components::{
    atoms::{form_title::TextTitle, logo::Logo, text_input::TextInput},
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

#[function_component(StartReset)]
pub fn start_reset() -> Html {

    let form: _ = use_state(|| StartResetRequest {
        email: "".into(),
    });
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onsubmit = {
        let form = form.clone();
        let validation_errors = validation_errors.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match form.validate() {
                Ok(()) => {
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
                   <form class={"space-y-4 md:space-y-6"} action="/api/auth/start-reset" method="post">
                        <TextInput label={"Email"} name={"email"} t={"email"} handle_onchange={email_change} handle_on_input_blur={onblur.clone()} errors={&*validation_errors}/>
                        <button type={"submit"} class={submit_button_classes()}>
                           <div class={"flex justify-center"}>
                               <span>{"Send reset link"}</span>
                           </div>
                       </button>
                   </form>
               </div>
           </div>
       </div>

    }
}
