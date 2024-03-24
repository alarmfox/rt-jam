use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html!(
    <div class={"flex flex-grow items-center text-center justify-center"}>
    <h1>{"NOT FOUND"} </h1>
    </div>
        )
}
