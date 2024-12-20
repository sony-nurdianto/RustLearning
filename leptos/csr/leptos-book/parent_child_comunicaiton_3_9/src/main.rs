use app::App;
use leptos::view;

mod app;
mod providing_ctx;

fn main() {
    leptos::mount_to_body(|| view! { <App /> })
}
