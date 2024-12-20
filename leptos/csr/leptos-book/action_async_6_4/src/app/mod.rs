use gloo_timers::future::TimeoutFuture;
use leptos::{
    component, create_action, create_node_ref, html::Input, view, IntoView, NodeRef, ReadSignal,
    RwSignal, SignalGet,
};
use uuid::Uuid;

async fn add_todo(text: &str) -> Uuid {
    _ = text;
    TimeoutFuture::new(1000).await;
    Uuid::new_v4()
}

#[component]
pub fn App() -> impl IntoView {
    let async_add_todo = create_action(|input: &String| {
        let input: String = input.to_owned();
        async move { add_todo(&input).await }
    });

    let submitted: RwSignal<Option<String>> = async_add_todo.input();
    let pending: ReadSignal<bool> = async_add_todo.pending();
    let todo_id: RwSignal<Option<Uuid>> = async_add_todo.value();

    let input_ref: NodeRef<_> = create_node_ref::<Input>();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("Chandelier");
                async_add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{
            move || pending.get().then(|| "Loading")
        }</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted.get())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending.get())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id.get())}</code>
        </p>
    }
}
