use leptos::{create_signal, mount_to_body, view, SignalUpdate};

fn main() {
    let (data, set_data) = create_signal(None::<String>);

    set_data.update(|n| *n = Some(String::from("something")));

    mount_to_body(move || view! {<h1>{data}</h1>});
}
