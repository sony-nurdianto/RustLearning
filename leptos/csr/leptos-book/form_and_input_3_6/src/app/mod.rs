use leptos::{
    component, create_node_ref, create_signal, event_target_value, html, logging::log, view,
    IntoView, NodeRef, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
};

/*

There are two important things to remember:

1.The input event fires on (almost) every change to the element, while the change event fires (more or less) when you unfocus the input. You probably want on:input, but we give you the freedom to choose.

2.The value attribute only sets the initial value of the input, i.e., it only updates the input up to the point that you begin typing. The value property continues updating the input after that. You usually want to set prop:value for this reason. (The same is true for checked and prop:checked on an <input type="checkbox">.)
 */

#[component]
pub fn ControlledInput() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input
            type="text"
            on:input=move |ev| { set_name.set(event_target_value(&ev)) }
            prop:value=name
        />
        <p>{name}</p>
    }
}

/*
1. Unlike in the controlled input example, we use value (not prop:value). This is because we’re just setting the initial value of the input, and letting the browser control its state. (We could use prop:value instead.)

2. We use node_ref=... to fill the NodeRef. (Older examples sometimes use _ref. They are the same thing, but node_ref has better rust-analyzer support.)
*/

#[component]
pub fn UncontrolledInput() -> impl IntoView {
    let (name, setname) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        match input_element.get() {
            Some(htme) => setname.set(htme.value()),
            None => log!("Failed To Set Name"),
        }
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="submit" />
        </form>
        <p>"Name is :" {name}</p>
    }
}

#[component]
pub fn TextArea() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <textarea
            prop:value=move || name.get_untracked()
            on:input=move |ev| { set_name.set(event_target_value(&ev)) }
        >

            {name.get_untracked()}
        </textarea>
        <p>{name}</p>
    }
}

#[component]
pub fn SelectComponent() -> impl IntoView {
    let (value, set_value) = create_signal(0i32);
    view! {
        <select
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value.set(new_value.parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button on:click=move |_| {
            set_value
                .update(|n| {
                    match n {
                        2 => *n = 0,
                        _ => *n += 1
                    }
                })
        }>"Next Option"</button>
    }
}
