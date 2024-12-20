use app::App;

mod app;

fn main() {
    leptos::mount_to_body(|| {
        leptos::view! { <App /> }
    })
}
