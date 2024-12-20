use html::Canvas;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(module = "/javascript_external/index.js")]
extern "C" {
    fn buildCanvas(lottie_path: &str, canvas: &HtmlCanvasElement);
}

#[component]
pub fn App() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    let lottie_data =
        include_str!("../test_asset/animations/7cfd6934-56db-4135-adbc-2649bf1d461d.json");

    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            buildCanvas(lottie_data, &canvas);
        }
    });

    view! {
        <canvas node_ref=canvas_ref  style="width: 800px; height:800px;"></canvas>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
