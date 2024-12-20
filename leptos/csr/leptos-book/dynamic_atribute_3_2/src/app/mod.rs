use leptos::{component, create_signal, view, IntoView, SignalGet, SignalUpdate};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1)
            }
            class:red=move || count.get() % 2 == 0
            class=("button-20",move || count.get() == 10)
        >
        "Click Me: "
        {count}
        </button>
    }
}
