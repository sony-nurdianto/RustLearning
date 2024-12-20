use app::App;

mod app;
mod components;

fn main() {
    leptos::mount_to_body(|| leptos::view! { <App /> })
}
