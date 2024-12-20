use leptos::{component, create_signal, view, IntoView, SignalSet, SignalUpdate};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (c, uc) = create_signal(0);

    view! {
        <div>
            <button on:click=move |_| {
                set_count.set(3);
            }>"Reactive Change Set: " {count}</button>
        </div>
        <br />
        <div>
            <button on:click=move |_| {
                uc.update(|n| *n += 1)
            }>"Reactive Change Update: " {c}</button>
        </div>
    }
}
