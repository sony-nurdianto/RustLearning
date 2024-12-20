use leptos::{component, view, IntoView};

use crate::components::{
    ExpensiveDynIteration, MemoizedIteration, MostEficientButCumbersome, NotWorkingIteration,
};

//not Render the data
#[component]
pub fn App() -> impl IntoView {
    view! {
        <NotWorkingIteration/>
        <br/>
        <ExpensiveDynIteration/>
        <br/>
        <MostEficientButCumbersome/>
        <br/>
        <MemoizedIteration/>
    }
}
