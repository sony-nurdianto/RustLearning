use std::process::id;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Ini Home"</h1>
    }
}

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <h1>"Ini Users"</h1>
        <A href="profile">"Users Profile"</A>
        <Outlet/>
    }
}

#[component]
pub fn UsersProfile() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let user_id = id().as_str().to_string();
    view! {
        <h1>"Ini Users Profile " {user_id}</h1>
    }
}
