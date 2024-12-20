use gloo_timers::future::TimeoutFuture;
use leptos::{
    component, create_resource, create_signal, view, IntoView, SignalGet, SignalSet, Transition,
};

async fn data_api(id: usize) -> String {
    TimeoutFuture::new(1000).await;
    match id {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        _ => "Data Not Found",
    }
    .to_string()
}

#[component]
pub fn App() -> impl IntoView {
    let (tab, set_tab) = create_signal(0usize);
    let user_data = create_resource(move || tab.get(), |tab| async move { data_api(tab).await });

    view! {
        <div>
            <button
                on:click=move |_| set_tab.set(0)
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab.set(1)
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab.set(2)
            >
                "Tab C"
            </button>
            {
                move || if user_data.loading().get() {
                    "User Data Loading Bapak"
                } else {"Someing else"}
            }
        </div>
        <Transition
            fallback=move||view! {<p>"Transition Fallback"</p>}
        >
            <p>{user_data}</p>
        </Transition>

    }
}
