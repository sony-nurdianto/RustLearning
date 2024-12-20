use leptos::{
    component, create_slice, event_target_value, use_context, view, IntoView, ReadSignal, RwSignal,
    Show, SignalGet, SignalUpdate, WriteSignal,
};

use crate::app::GlobalState;

#[component]
pub fn SetterButton(set_count: WriteSignal<i32>) -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1)
            }>"Increment Global State"</button>
        </div>
    }
}

#[component]
pub fn FancyMath() -> impl IntoView {
    let count = use_context::<ReadSignal<i32>>().expect("there to be `count` signal provided");

    let is_even = move || count.get() % 2 == 0;

    view! {
        <div>
            <h4>
                "The number "<strong>{count}</strong>
                <Show
                    when=move || is_even()
                    fallback=move || view! { <span>" Is Odd"</span> }.into_view()
                >
                    <span>" Is Even"</span>
                </Show>
            </h4>

        </div>
    }
}

#[component]
pub fn ListItem() -> impl IntoView {
    let count = use_context::<ReadSignal<i32>>().expect("There to be `count` signal provided");

    let sequares = move || {
        (0..count.get())
            .map(|n| view! { <li>{n}<sup>"2"</sup> " is " {n * n}</li> })
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <ul>{sequares}</ul>
        </div>
    }
}

#[component]
pub fn GlobalStateCounter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    let (count, set_count) = create_slice(state, |state| state.count, |state, n| state.count = n);
    view! {
        <div>
            <h4>"Counting From: "{count}</h4>
            <button on:click=move |_| set_count.set(count.get() + 1)>"Count"</button>
        </div>
    }
}

#[component]
pub fn GlobalStateInput() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    let (name, set_name) = create_slice(
        state,
        |state| state.name.clone(),
        |state, name| state.name = name,
    );

    view! {
        <div>
            <h4>"From Input:"</h4>
            <h4>{name}</h4>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| set_name.set(event_target_value(&ev))
            />
            <button on:click=move |_| set_name.set(String::new())>"Clear"</button>
        </div>
    }
}
