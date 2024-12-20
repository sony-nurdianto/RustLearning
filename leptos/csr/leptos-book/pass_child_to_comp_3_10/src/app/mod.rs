use leptos::{component, view, Children, CollectView, IntoView};

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}
        <h2>"Render Children"</h2>
        {children()}
    }
}

#[component]
fn ManipulateChildren(children: Children) -> impl IntoView {
    let child = children()
        .nodes
        .into_iter()
        .map(|c| view! {<li>{c}</li>})
        .collect_view();

    view! {
        <h2>"Let's Manipulate Children"</h2>
        <ul>
        {child}
        </ul>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TakesChildren render_prop=|| view!{<p>"This is Render Prop"</p>}>
            <p>"This Is Children"</p>
            <p>"Let's Do More"</p>
        </TakesChildren>
        <br/>
        <ManipulateChildren>
        "apple"
        "banana"
        "cerry"
        </ManipulateChildren>
    }
}
