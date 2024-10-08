use yew::{function_component, html, Html, Properties};

use crate::components::atoms::footer::Footer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}

#[function_component(SimpleLayout)]
pub fn simple_layout(props: &Props) -> Html {
    html!(
        <div class={"flex flex-col h-screen"}>
            <main class={"flex-grow mt-auto overflow-y-auto mb-1"}>
                {props.children.clone() }
            </main>
            <Footer />
        </div>
    )
}
