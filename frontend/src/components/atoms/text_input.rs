use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    pub label: AttrValue,
    pub required: bool,
}

#[function_component(TextInput)]
pub fn text_input(Props { name, label , required}: &Props) -> Html {
    html! {
    <div>
    <label for={name} class={"block mb-2 text-sm font-medium text-gray-900 dark:text-white"}>{label}</label>
    <input required={*required} {name} id={name} class={"bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"} placeholder={label} />
    </div>
    }
}
