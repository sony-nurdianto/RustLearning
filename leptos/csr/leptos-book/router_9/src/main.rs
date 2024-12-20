use app::App;

mod app;
mod pages;

fn main() {
    leptos::mount_to_body(|| leptos::view! {<App/>})
}
