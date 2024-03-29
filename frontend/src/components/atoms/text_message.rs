use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub message: AttrValue,
}

#[function_component(TextTitle)]
pub fn text_title(Props { message }: &Props) -> Html {
    html! {
        <div class="p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400" role="alert">
            {Change a few things up and try submitting again.
        </div>:Ex
    }
}
