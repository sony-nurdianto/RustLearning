use app::{ControlledInput, SelectComponent, TextArea, UncontrolledInput};

mod app;

fn main() {
    leptos::mount_to_body(|| {
        leptos::view! {
            <ControlledInput/>
            <br/>
            <UncontrolledInput/>
            <br/>
            <TextArea/>
            <br/>
            <SelectComponent/>
        }
    })
}
