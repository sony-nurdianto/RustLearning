use leptos::{
    component, create_signal, ev::MouseEvent, view, IntoView, Show, SignalGet, SignalUpdate,
    WriteSignal,
};

use crate::providing_ctx::{ProvidingContext, UsingContext};

#[component]
fn PassWriteSignal(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|con| *con = !*con)>"Toggled"</button> }
}

#[component]
fn EventListener() -> impl IntoView {
    view! { <button>"Event Listener Toggled"</button> }
}

#[component]
fn CallBackM<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <button on:click=on_click>"Toggle"</button> }
}

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    let (an_tog, set_an_tog) = create_signal(false);
    let (ev_to, set_ev_tog) = create_signal(false);

    let com_pos = r#"
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    "#;

    view! {
        <UsingContext style=com_pos.to_string() />
        <ProvidingContext style=com_pos.to_string() />
        <div style=com_pos>
            <Show
                when=move || { ev_to.get() }
                fallback=|| {
                    view! { <h1>"Bonjour!"</h1> }
                }
            >
                <h1>"Hallo"</h1>
            </Show>
            <EventListener on:click=move |_| set_ev_tog.update(|b: &mut bool| *b = !*b) />
        </div>
        <div style=com_pos>
            <Show when=move || { an_tog.get() } fallback=|| view! { <h1>"Wie Geht ?"</h1> }>
                <h1>"Es Geht's"</h1>
            </Show>
            <br />
            <CallBackM on_click=move |_| set_an_tog.update(|n| *n = !*n) />
        </div>
        <div style=com_pos>
            <Show when=move || { toggled.get() } fallback=|| view! { <h1>"Off"</h1> }>
                <h1>"On"</h1>
            </Show>
            <br />
            <PassWriteSignal setter=set_toggled />
        </div>
    }
}
