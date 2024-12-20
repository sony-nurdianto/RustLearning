use leptos::{component, view, IntoView, ReadSignal, Signal};
use std::marker;

//optional generic props
#[component]
pub fn ProgressOptGen(
    #[prop(default = 100)] max: u16,
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32 + 'static>>,
) -> impl IntoView {
    let id: &str = "progress_opt_gen";

    progress.map(|p| {
        view! {
            <label for=id>"Optional Generic Props: "</label>
            <progress
                id=id
                max=max
                value=p
            />
        }
    })
}

//into props
#[component]
pub fn ProgressInto(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    let id: &str = "into_props";
    view! {
        <label for=id>"Into Props: "</label>
        <progress
            id=id
            max=max
            value=progress
        />
    }
}

#[component]
pub fn SizeOf<T: Sized>(#[prop(optional)] _ty: marker::PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

#[component]
pub fn ProgressAnother<T>(#[prop(optional, default = 100)] max: u16, progress: T) -> impl IntoView
where
    T: Fn() -> i32 + 'static,
{
    let id: &str = "generic_props";

    view! {
        <label for=id>"Generic Props: "</label>
        <progress
            id=id
            max= max
            value=progress
        />
    }
}

#[component]
pub fn Progress(
    progress: ReadSignal<i32>,
    #[prop(optional, default = 100)] max: u16,
) -> impl IntoView {
    let id: &str = "Optional Props";

    view! {
        <label for=id>"Optional Props: "</label>
        <progress
            id=id
            max=max
            value=progress
        />
    }
}
