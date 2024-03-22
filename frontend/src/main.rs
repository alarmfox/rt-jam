mod layout;

use layout::Layout;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/change-password")]
    ChangePassword,
    
    #[at("/not-found")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Hello world" } </h1>
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Login" } </h1>
    }
}

#[function_component(ChangePassword)]
fn change_passsword() -> Html {
    html! {
    <h1 class="bg-purple-600"> { "Change password" } </h1>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Layout>
                <h1>{ "Home" }</h1> 
            </Layout>
        },
        Route::Login => html! {
            <Login/>
        },
        Route::ChangePassword=> html! {
            <ChangePassword/>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
