use leptos::{
    component, create_signal, logging::log, provide_context, use_context, view, IntoView, Show,
    SignalGet, SignalUpdate, WriteSignal,
};

#[component]
pub fn UsingContext(#[prop(optional)] style: String) -> impl IntoView {
    let (tog, set_tog) = create_signal(false);

    provide_context(set_tog);

    view! {
        <div style=style>
            <Show when=move || tog.get() fallback=|| view! { <h1>"From Branch"</h1> }>
                <h1>"From Root"</h1>
            </Show>
            <ToggledButton />
        </div>
    }
}

#[component]
fn ToggledButton() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>();

    view! {
        <button on:click=move |_| {
            if let Some(s) = setter {
                s.update(|b| *b = !*b)
            }
            log!("Failled to Use Context");
        }>"Tog Toga Toga Toga Toga"</button>
    }
}
#[component]
pub fn ProvidingContext(#[prop(optional)] style: String) -> impl IntoView {
    let (tog, set_tog) = create_signal(false);
    view! {
        <div style=style>
            <Show when=move || tog.get() fallback=|| view! { <h1>"Raja Mexico"</h1> }>
                <h1>"Raja Spanyol"</h1>
            </Show>
            <Layout setter=set_tog />
        </div>
    }
}

#[component]
fn Layout(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <Content setter=setter /> }
}

#[component]
fn Content(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|b: &mut bool| *b = !*b)>"Let's Do it"</button> }
}
