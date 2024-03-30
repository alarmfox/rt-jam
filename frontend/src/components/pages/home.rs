use yew::prelude::*;

use crate::components::webtransport::client::Model;

#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <Model />
    }
}
