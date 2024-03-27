use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::atoms::class::{input_error_classes, label_classes, text_input_classes};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    pub label: AttrValue,

    #[prop_or(AttrValue::from("text"))]
    pub t: AttrValue,

    #[prop_or(true)]
    pub required: bool,

    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        t,
        name,
        label,
        required,
        handle_on_input_blur,
        handle_onchange,
        errors,
    } = props;
    let val_errors = errors.borrow();
    let true_errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match true_errors.get(&name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let onchange = {
        let handle_onchange = handle_onchange.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            handle_onchange.emit(value);
        })
    };

    let onblur = {
        let handle_on_input_blur = handle_on_input_blur.clone();
        let input_name = name.clone();
        Callback::from(move |e: FocusEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name.clone().to_string(), value));
        })
    };
    html! {
    <div>
        <label for={name} class={label_classes()}>{label}</label>
        if t == "password" {
            <PasswordInput name={name} handle_onchange={handle_onchange.clone()} handle_on_input_blur={handle_on_input_blur.clone()} errors={errors.clone()}/>
        }else {
            <input type={t} required={*required} {name} id={name} class={text_input_classes()} placeholder={label}
                onchange={onchange}
                onblur={onblur}

            />
        }
        <span class={input_error_classes()}>
            {error_message}
        </span>
    </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PasswordProp {
    pub name: AttrValue,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(PasswordInput)]
pub fn password_input(
    PasswordProp {
        name,
        handle_on_input_blur,
        handle_onchange,
        errors,
    }: &PasswordProp,
) -> Html {
    let button_classes_open = classes!(
        "focus:outline-none",
        "-ml-12",
        "text-gray-200",
        "fa-eye",
        "fa-solid"
    );
    let button_classes_closed = classes!(
        "focus:outline-none",
        "-ml-12",
        "text-gray-200",
        "fa-eye-slash",
        "fa-solid"
    );
    let password_hidden = use_state(|| true);

    let callback = {
        let password_hidden = password_hidden.clone();
        Callback::from(move |_| password_hidden.set(!*password_hidden))
    };
    
    let onchange = {
        let handle_onchange = handle_onchange.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            handle_onchange.emit(value);
        })
    };

    let onblur = {
        let handle_on_input_blur = handle_on_input_blur.clone();
        let input_name = name.clone();
        Callback::from(move |e: FocusEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name.clone().to_string(), value));
        })
    };

    html! {
          <div class={"flex items-center"}>
            if *password_hidden {
                <input type={ "password"} onchange={onchange} onblur={onblur} {name} id={name} placeholder={"••••••••"} class={text_input_classes()} required={true}/>
                <button type={"button"} onclick={callback} class={button_classes_open}></button>
            } else {
                <input type={"text"} onchange={onchange} onblur={onblur} {name} id={name} placeholder={"••••••••"} class={text_input_classes()} required={true}/>
                <button type={"button"} onclick={callback} class={button_classes_closed}></button>
            }

        </div>
    }
}
