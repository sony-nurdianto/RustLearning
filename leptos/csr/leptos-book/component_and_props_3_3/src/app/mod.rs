use std::usize;

use leptos::{
    component, create_signal, logging::log, view, IntoView, Signal, SignalGet, SignalUpdate,
};
use progress_component::{Progress, ProgressAnother, ProgressInto, ProgressOptGen, SizeOf};

pub mod progress_component;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;

    let mut progress_max: u16 = 100;

    view! {
        <ProgressOptGen progress=Box::new(double_count)/>
        <br/>
        <ProgressInto
            progress=Signal::derive(double_count)
        />
        <br/>
        <SizeOf<String>/>
        <br/>
        <SizeOf<usize>/>
        <br/>
        <Progress
            max=progress_max
            progress=count
        />
        <br />
        <ProgressAnother
            max=progress_max
            progress=double_count
        />
        <br/>
        <button on:click=move |_| {
            log!("Takan Pernah Berhenti Untuk Selalu Percaya Walau Harus Menunggu 1000 Tahun Lamanya");
            set_count
                .update(|n: &mut i32| {
                    log!("{}",n);
                    match n {
                        0 => *n += 1,
                        _ => {
                            if *n == 10 {
                                progress_max = 50;
                            }
                            *n *= 2
                        },
                    }
                })
        }>"Adding Proggress"</button>
    }
}
