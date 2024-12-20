use app::App;
use leptos::view;

mod app;

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <h1>"Hallo Hier Ist Leptos"</h1>
                <br/>
            <App />
        }
    })
}
