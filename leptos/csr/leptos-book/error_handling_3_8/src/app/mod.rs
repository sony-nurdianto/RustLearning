use leptos::{
    component, create_signal, ev::Event, event_target_value, view, CollectView, ErrorBoundary,
    IntoView, SignalGet, SignalSet,
};

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev: Event| set_value.set(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
