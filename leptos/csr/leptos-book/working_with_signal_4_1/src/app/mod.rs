use gloo_timers::future::TimeoutFuture;
use std::{future::Future, usize};

use leptos::{
    component, create_effect, create_memo, create_resource, create_signal, ev::Event,
    event_target_value, logging::log, view, watch, For, IntoView, ReadSignal, Show, SignalGet,
    SignalSet, SignalUpdate, SignalWith,
};

async fn load_data() -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1000).await;
    10
}

#[component]
fn CancelablleTrackingWithWatch() -> impl IntoView {
    let (num, set_num) = create_signal(0i32);

    let (count, set_count) = create_signal(0);

    let async_data = create_resource(
        || (),
        |_| async move {
            log!("loading data from API");
            load_data().await
        },
    );

    let stop = watch(
        move || num.get(),
        move |num, prev_num, letsee| {
            log!(
                "Number: {}; prev: {:?}; somethingElse: {:?}",
                num,
                prev_num,
                letsee
            );
        },
        false,
    );

    view! {

        <Show
            when=move || async_data.get().is_some()
            fallback= move || view! {<h1>"Loading ..."</h1>}
        >
            <h1>"This is Async Data : "{move || async_data.get()}</h1>

        </Show>
                <br/>
        <button
            on:click=move |_| set_count.update(|n| *n += 1)
        >
            "Update The data"
        </button>
        <br/>
        <h2>{num}</h2>
        <button
            on:click=move |_| set_num.update(|num| *num += 1)
        >"Let's See"</button>
        <button
            on:click=move |_| stop()
        >"Stop Watching Perv !!!!"</button>
    }
}

#[component]
fn LearningCreateEffect() -> impl IntoView {
    let (a, set_a) = create_signal(0);
    let (b, set_b) = create_signal(0);

    let c = create_memo(move |_| a.get() + b.get());

    create_effect(move |_| {
        log!("Value: {}", a.get());
    });

    view! {
        <h2>"I am C : "{c}</h2>
        <button
            on:click=move |_| set_a.update(|a| *a += 1)
        >
            "Show The Effect of a : " {a}
        </button>
        <br/>
        <button
            on:click=move |_| set_b.update(|b| *b += 1)
        >
            "Show The Effect of b : " {b}
        </button>
    }
}

#[component]
fn MyComponent() -> impl IntoView {
    // Create two signals with initial values
    let (num1, set_num1) = create_signal(10);
    let (num2, set_num2) = create_signal(20);

    let sum = create_memo(move |_| num1.get() + num2.get());

    view! {
        <div>
            <p>"Number 1: " {move || num1.get()}</p>
            <p>"Number 2: " {num2}</p>
            <p>"Sum: " {sum}</p>
            <button on:click=move |_| set_num1.update(|n| *n += 1)>"Increment Num1"</button>
            <button on:click=move |_| set_num2.update(|n| *n += 1)>"Increment Num2"</button>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (name, set_name) = create_signal(Vec::<String>::new());

    create_effect(move |_| {
        if name.with(|n| n.is_empty()) {
            set_name.update(|val| val.push("mamen".to_string()))
        }
    });

    view! {
        <div>
            <input
                type="text"
                on:input=move |ev: Event| set_input.set(event_target_value(&ev))
                prop:value=input
            />
            <button on:click=move |_| {
                set_name.update(|n| n.push(input.get()));
                set_input.update(|val_input| val_input.clear());
            }>"Push Button"</button>
        </div>
        <ul>
            <For
                each=move || name.get().into_iter().enumerate()
                key=|(_, key)| key.to_string()
                children=move |(_, val): (usize,String)| {
                    view! { <li>{val}</li> }
                }
            />
        </ul>
        <div>
            <MyComponent/>
        </div>
        <br/>
        <div>
            <LearningCreateEffect/>
        </div>
        <div>
           <CancelablleTrackingWithWatch/>
        </div>
    }
}
