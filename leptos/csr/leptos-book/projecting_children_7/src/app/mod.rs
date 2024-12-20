use gloo_timers::future::TimeoutFuture;
use leptos::{
    component, create_resource, create_signal, logging::log, store_value, view, ChildrenFn,
    IntoView, Resource, Show, SignalGet, SignalUpdate, Suspense,
};

async fn signin(val: bool) -> bool {
    TimeoutFuture::new(1000).await;
    val
}

#[component]
pub fn LoggedIn<F, IV>(
    fallback: F,
    children: ChildrenFn,
    isloggin: Resource<bool, bool>,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let fallback = store_value(fallback);
    let children = store_value(children);
    view! {
        <Suspense fallback=move || view! { <p>"Loading User Login State ..."</p> }>
            <Show
                when=move || match isloggin.get() {
                    Some(val) => {
                        log!("This is From When True");
                        val
                    }
                    None => {
                        log!("This is From When False");
                        false
                    }
                }
                fallback=move || {
                    fallback
                        .with_value(|fallback| {
                            log!("This is From Fallback Show");
                            fallback()
                        })
                }
            >
                {children
                    .with_value(|children| {
                        log!("This is From Children Show");
                        children()
                    })}
            </Show>
        </Suspense>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (login, set_login) = create_signal(false);

    let user_state = create_resource(
        move || login.get(),
        |login| async move { signin(login).await },
    );

    view! {
        <div>
            <h1>"Welcome Home"</h1>
            <br />
            <LoggedIn fallback=|| view! { <p>"Not Loggin"</p> } isloggin=user_state>
                <p>"Alredy Loggin"</p>
            </LoggedIn>
        </div>
        <br />
        <div>
            <button on:click=move |_| set_login.update(|b| *b = !*b)>Loggin</button>
        </div>
    }
}
