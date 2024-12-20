use leptos::{component, create_signal, view, IntoView, Show, SignalGet, SignalUpdate};

#[component]
pub fn App() -> impl IntoView {
    let (ja, set_ja) = create_signal(true);

    view! {
        <Show
            when=move||{ja.get()}
            fallback=|| view! {<h1>"Nein,Ich bin es, Leptos"</h1>}.into_any()
        >
            <h1>"c'est moi Leptos"</h1>
        </Show>
        <br/>
        <button
            on:click=move|_| {
                set_ja.update(|b| *b = !*b)
            }
        >
            "Change The Title"
        </button>
    }
}
