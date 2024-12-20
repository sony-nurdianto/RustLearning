use leptos::{component, view, IntoView};
use static_iteration::{
    DynamicList, IstSieStatischOderNicht, StaticIteration, StaticIterationColView,
};

pub mod static_iteration;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StaticIteration />
            <br/>
        <StaticIterationColView/>
            <br/>
        <IstSieStatischOderNicht/>
            <br/>
        <DynamicList initial_length=5/>
    }
}
