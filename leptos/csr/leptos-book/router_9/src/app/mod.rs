use leptos::{component, view, IntoView, Show};
use leptos_router::{Route, Router, Routes};

use crate::pages::{Home, Users, UsersProfile};

#[component]
pub fn App() -> impl IntoView {
    let has_permission = move || true;

    view! {
        <Router>
            <Routes>
                // <Route path="/" view=move || view!{
                //     <Show when= move || has_permission() fallback=|| view! {<p>"Access Deninend"</p>}>
                //         <UsersProfile/>
                //     </Show>
                // }>
                //  <Route path="/" view=Home/>
                // </Route>

                 // <Route path="/" view=Home/>
                 <Route path="/users" view=Users>
                    <Route path=":id" view=UsersProfile/>
                    <Route path="" view=||view!{<h1>"No User"</h1>}/>
                 </Route>
                // <Route path="/*any" view=|| view! {<h1>"Not Found"</h1>}/>
            </Routes>
        </Router>
    }
}
