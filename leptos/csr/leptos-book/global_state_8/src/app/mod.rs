use leptos::{component, create_rw_signal, create_signal, provide_context, view, IntoView};

use crate::components::{FancyMath, GlobalStateCounter, GlobalStateInput, ListItem, SetterButton};

#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub count: i32,
    pub name: String,
}

#[component]
fn Option2() -> impl IntoView {
    let (count, set_count) = create_signal(0i32);
    provide_context(count);

    view! {
        <div>
            <h1>"Passing Signal through Context"</h1>
            <div>
                <SetterButton set_count=set_count />
                <FancyMath />
                <ListItem />
            </div>
        </div>
    }
}

#[component]
fn Option3() -> impl IntoView {
    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        <div>
            <h1>"Global State Struct Way"</h1>
            <div>
                <GlobalStateCounter />
                <br />
                <GlobalStateInput />
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let style = r#"
          display: flex;
          justify-content: space-around;
        "#;

    view! {
        <div style=style>
            <Option2 />
            <Option3 />
        </div>
    }
}
