use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Hello world" } </h1>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
