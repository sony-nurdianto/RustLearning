use app::App;
use leptos::view;

mod app;
mod components;
mod models;

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
