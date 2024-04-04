use app::App;

mod app;
mod components;
mod store;
mod utils;

pub const WEBTRANSPORT_HOST: &str = concat!("https://127.0.0.1:4433", "/room");

fn main() {
    yew::Renderer::<App>::new().render();
}
