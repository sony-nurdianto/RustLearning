use app::App;
use leptos::view;

mod app;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}
