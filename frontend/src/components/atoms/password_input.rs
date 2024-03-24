use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,

    #[prop_or(true)]
    pub show_reveal_password: bool,
}

#[function_component(PasswordInput)]
pub fn password_input(
    Props {
        label,
        name,
        show_reveal_password,
    }: &Props,
) -> Html {
    let label_classes = classes!(
        "block",
        "mb-2",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-white"
    );

    let input_classes = classes!(
        "bg-gray-50",
        "border",
        "border-gray-300",
        "text-gray-900",
        "sm:text-sm",
        "rounded-lg",
        "focus:ring-primary-600",
        "focus:border-primary-600",
        "block",
        "w-full",
        "p-2.5",
        "dark:bg-gray-700",
        "dark:border-gray-600",
        "dark:placeholder-gray-400",
        "dark:text-white",
        "dark:focus:ring-blue-500",
        "dark:focus:border-blue-500"
    );

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

    html! {
        <>
          <label for={name} class={label_classes}>{label}</label>
          <div class={"flex items-center"}>
           if *show_reveal_password {
            if *password_hidden {
                <input type={"password"} {name} id={name} placeholder={"••••••••"} class={input_classes} required={true}/>
                <button type={"button"} onclick={callback} class={button_classes_open}></button>
            } else {
                <input type={"text"} {name} id={name} placeholder={"••••••••"} class={input_classes} required={true}/>
                <button type={"button"} onclick={callback} class={button_classes_closed}></button>
            }
            } else {
                <input type={"password"} {name} id={name} placeholder={"••••••••"} class={input_classes} required={true}/>
            }
          </div>
       </>
    }
}
