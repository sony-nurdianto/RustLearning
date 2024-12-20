use leptos::{component, create_signal, view, CollectView, For, IntoView, SignalGet, SignalUpdate};

#[component]
pub fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=move||counters.get()
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

//The fact that the list is static doesn’t mean the interface needs to be static.
//You can render dynamic items as part of a static list.
#[component]
pub fn IstSieStatischOderNicht() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <p
                        style="cursor: pointer"
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        "Brechnen Von "{count}
                    </p>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>
            {counter_buttons}
        </ul>
    }
}

//with collect view method
#[component]
pub fn StaticIterationColView() -> impl IntoView {
    let values: Vec<&str> = vec![
        "Kukira kita akan Bersama",
        "Begitu banyak yang sama",
        "Latarku dan Latarmu",
        "Kukira takan ada kendala",
        "Kukira ini kan mudah",
        "Kau aku jadi kita",
    ];
    view! {
      <p>{values.clone()}</p>
      <ul>
        {
            values.into_iter()
                .map(|n| view!{<li>{n}</li>})
                .collect_view()
        }
      </ul>
    }
}

//normal collect methos
#[component]
pub fn StaticIteration() -> impl IntoView {
    let values: Vec<i32> = vec![1, 2, 3];
    view! {
        <p>{values.clone()}</p>
        <ul>
            {
                values.into_iter()
                    .map(|n| view! { <li>{n}</li> })
                    .collect::<Vec<_>>()
            }
        </ul>
    }
}
