use yew::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    let classes = classes!(
        "flex",
        "items-center",
        "mb-6",
        "text-2xl",
        "font-semibold",
        "text-gray-900",
        "dark:text-white"
    );
    html! {
    <a href="#" class={classes}>
      <img class={"w-8 h-8 mr-2"} src={"/static/logo.svg"} alt={"logo"}/>
          {"RT-Jam"}
    </a>
      }
}
