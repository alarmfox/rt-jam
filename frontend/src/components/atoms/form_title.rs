use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub message: AttrValue,
}

#[function_component(TextTitle)]
pub fn text_title(props: &Props) -> Html {
    let classes = classes!(
        "text-xl",
        "font-bold",
        "leading-tigh",
        "tracking-tight",
        "text-gray-900",
        "md:text-2xl",
        "dark:text-white"
    );
    html! {
      <h1 class={classes}>
            {props.message.clone()}
      </h1>
    }
}
