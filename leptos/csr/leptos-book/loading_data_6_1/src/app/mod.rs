use gloo_timers::future::TimeoutFuture;
use leptos::{
    component, create_memo, create_resource, create_signal, logging::log, view, Await, IntoView,
    SignalGet, SignalUpdate, Suspense,
};

async fn load_data_w_arg(value: i32) -> i32 {
    TimeoutFuture::new(1000).await;
    value * 10
}

async fn load_data() -> String {
    TimeoutFuture::new(1000).await;
    String::from("This is Async Load Data Baby")
}

async fn fetch_monkey(m: i32) -> i32 {
    TimeoutFuture::new(2000).await;
    m * 2
}

#[component]
fn SymplyfiesNonReactiveDataWAwait() -> impl IntoView {
    view! {
    <Await
           future=|| fetch_monkey(3)
           let:data
       >
           <h1>{*data} " little monkeys, jumping on the bed."</h1>
       </Await>
       }
}

#[component]
fn AwaitMultipleDataWSuspense() -> impl IntoView {
    let (count, _) = create_signal(1i32);
    let (count2, _) = create_signal(1i32);

    let a = create_resource(
        move || count.get(),
        |count| async move { load_data_w_arg(count).await },
    );
    let b = create_resource(
        move || count2.get(),
        |count2| async move { load_data_w_arg(count2).await },
    );

    let sum2 = create_memo(move |_| match (a.get(), b.get()) {
        (Some(c1), Some(c2)) => c1 + c2,
        _ => 0,
    });

    view! {
        <div>
            <h1>"Verbose Example"</h1>
            {move || match (a.get(), b.get()) {
                (Some(a), Some(b)) => {
                    let sum = create_memo(move |_| a + b);
                    view! {
                        <div>
                            <h2>"This Is A "{a}</h2>
                            <br />
                            <h2>"This is B "{b}</h2>
                            <br />
                            <h2>"This is Sum "{sum}</h2>
                        </div>
                    }
                        .into_view()
                }
                _ => view! { <h2>"Loading ...."</h2> }.into_view(),
            }}
        </div>
        <br />
        <div>
            <h1>"This Is With Suspense Function"</h1>
            <br />
            <Suspense fallback=move || view! { <h1>"Loading ..."</h1> }>
                <h2>"This Is A "{a}</h2>
                <br />
                <h2>"This is B "{b}</h2>
                <br />
                <h2>"This is Sum1 "{move || create_memo(move |_| {
                    let num1 = a.get().map(|a| a).unwrap_or(0);
                    let num2 = b.get().map(|b| b).unwrap_or(0);
                    num1 + num2
                })}</h2>
                <br/>
                <h2>"This is Sum2 "{sum2}</h2>
            </Suspense>
        </div>
    }
}

#[component]
fn NonReactiveLoadDataOnce() -> impl IntoView {
    let async_data = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <div>
            {move || match async_data.get() {
                Some(data) => {
                    log!("This is processs when data loaded");
                    view! { <h1>{data}</h1> }
                }
                None => {
                    log!("This is processs when load data");

                    view! { <h1>"Loading ..."</h1> }
                }
            }}
        </div>
    }
}

#[component]
fn ReactiveMultipleLoadData() -> impl IntoView {
    let (count, set_count) = create_signal(0i32);

    let async_data = create_resource(
        move || count.get(),
        |value| async move {
            log!("Loading data ! : {}", value);
            load_data_w_arg(value).await
        },
    );

    view! {
        <div>
            <h2>"This is Load Data Async: "{async_data}</h2>
            <br />
            <button on:click=move |_| set_count.update(|n| *n += 1)>"Count button"</button>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <NonReactiveLoadDataOnce />
        <br />
        <ReactiveMultipleLoadData />
        <br />
        <AwaitMultipleDataWSuspense />
        <br />
        <SymplyfiesNonReactiveDataWAwait/>

    }
}
