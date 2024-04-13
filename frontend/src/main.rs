use app::App;

mod app;
mod components;
mod store;
mod utils;

pub const WEBTRANSPORT_HOST: &str = concat!("https://192.168.1.2:4433", "/room");

fn main() {
    yew::Renderer::<App>::new().render();
}
