use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html!(
        <div class={"flex flex-col h-screen"}>
        <main class={"flex-grow mt-auto overflow-y-auto mb-1"}>
            {props.children.clone() }
        </main>
        </div>
    )
}
