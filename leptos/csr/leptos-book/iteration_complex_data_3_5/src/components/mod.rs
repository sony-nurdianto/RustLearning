use leptos::{
    component, create_memo, create_rw_signal, create_signal, logging, view, For, IntoView, Memo,
    SignalGet, SignalUpdate, SignalWith,
};

use crate::models::{DatabaseEntry, DatabaseEntryRwS};

//solutions number one Change The Key
//change the key so that it always updates when the data structure changes
/*

Pros:
    This is very easy. We can make it even easier by deriving PartialEq, Eq, and Hash on DatabaseEntry, in which #case we could just key=|state| state.clone().

Cons:
    This is the least efficient of the three options. Every time the value of a row changes, it throws out the previous <p> element and replaces it with an entirely new one. Rather than making a fine-grained update to the text node, in other words, it really does rerender the entire row on every change, and this is expensive in proportion to how complex the UI of the row is.

You’ll notice we also end up cloning the whole data structure so that <For/> can hold onto a copy of the key. For more complex structures, this can become a bad idea fast!
*/
#[component]
pub fn ExpensiveDynIteration() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 30,
        },
    ]);

    view! {
        <h3>"Expensive Dynamic Iteration component"</h3>
        <button
            on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2
                    }
                });
                logging::log!("{:?}",data.get());
        }>
            "Updata Value"
        </button>
        <For
            each=move || data.get()
            key=|state| (state.key.clone(),state.value)
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}

//solutions Number Two Nested Signal
/*
Pros
    This is the most efficient option, and fits directly with the rest of the mental model of the framework: values that change over time are wrapped in signals so the interface can respond to them.

Cons
    Nested reactivity can be cumbersome if you’re receiving data from an API or another data source you don’t control, and you don’t want to create a different struct wrapping each field in a signal.
 */
#[component]
pub fn MostEficientButCumbersome() -> impl IntoView {
    let (data, _) = create_signal(vec![
        DatabaseEntryRwS {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntryRwS {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntryRwS {
            key: "baz".to_string(),
            value: create_rw_signal(30),
        },
    ]);

    view! {
    <h3>"Eficient Dynamic Iteration component But Cumbersome"</h3>
            <button
                on:click=move |_| {
                    data.with(|data| {
                        for row in data {
                            row.value.update(|value| *value *= 2);
                        }
                    });
                    logging::log!("{:?}",data.get());
            }>
                "Updata Value"
            </button>
            <For
                each=move || data.get()
                key=|state| state.key.clone()
                let:child
            >
                <p>{child.value}</p>
            </For>
        }
}

//Solutions Number 3 Memoized Slice
/*
 Pros:
    We get the same fine-grained reactivity of the signal-wrapped version, without needing to wrap the data in signals.

Cons:
    It’s a bit more complex to set up this memo-per-row inside the <For/> loop rather than using nested signals. For example, you’ll notice that we have to guard against the possibility that the data[index] would panic by using data.get(index), because this memo may be triggered to re-run once just after the row is removed. (This is because the memo for each row and the whole <For/> both depend on the same data signal, and the order of execution for multiple reactive values that depend on the same signal isn’t guaranteed.)

Note also that while memos memoize their reactive changes, the same calculation does need to re-run to check the value every time, so nested reactive signals will still be more efficient for pinpoint updates here.
 */
#[component]
pub fn MemoizedIteration() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    view! {
            <h3>"Good Iteration Component But Complex to Set Up Memo-Per-Row"</h3>
            <button
                on:click=move |_| {
                    set_data.update(|data| {
                        for row in data {
                            row.value *= 2
                        }
                    });
                    logging::log!("{:?}",data.get());
            }>
                "Updata Value"
            </button>
    <For
        each=move || data.get().into_iter().enumerate()
        key=|(_, state)| state.key.clone()
        children=move |(index, _)| {
            let value: Memo<i32>= create_memo(move |_| {
               data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
            });
            view! {
                <p>{value}</p>
            }
        }
    />
        }
}

#[component]
pub fn NotWorkingIteration() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    view! {
        <h3>"Not Working Iteration Component"</h3>
        <button
            on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2
                    }
                });
                logging::log!("{:?}",data.get());
        }>
            "Updata Value"
        </button>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}
