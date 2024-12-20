use full_palette::BLACK;
use leptos::{html::Canvas, *};
use plotters::prelude::{Circle, *};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[component]
pub fn MultiplePlot() -> impl IntoView {
    let multi_plot_ref = create_node_ref::<Canvas>();

    let multiplot = move |plot_ref: &HtmlCanvasElement| {
        let canvas: HtmlCanvasElement = plot_ref
            .unchecked_ref::<web_sys::HtmlCanvasElement>()
            .clone();

        let root_are = CanvasBackend::with_canvas_object(canvas)
            .expect("Failed to initialize the canvas backend")
            .into_drawing_area();

        root_are.fill(&WHITE).expect("Failed to clear the canvas");

        let (upper, lower) = root_are.split_vertically(512);

        let x_axis = (-3.4f32..3.4).step(0.1);

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)
            .unwrap();

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()
            .unwrap();

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))
            .unwrap()
            .label("Sine")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.cos())),
            &BLUE,
        ))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        cc.configure_series_labels()
            .border_style(BLACK)
            .draw()
            .unwrap();

        cc.draw_series(PointSeries::of_element(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        ))
        .unwrap(); // Ensure to handle the Result here as well

        let drawing_areas = lower.split_evenly((1, 2));

        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
                .unwrap();

            cc.configure_mesh()
                .x_labels(5)
                .y_labels(3)
                .max_light_lines(4)
                .draw()
                .unwrap();

            cc.draw_series(LineSeries::new(
                (-1f32..1f32)
                    .step(0.01)
                    .values()
                    .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                &BLUE,
            ))
            .unwrap();
        }

        root_are.present().expect("Failed to present the chart");
    };

    // Effect that triggers chart plotting when the component is mounted
    create_effect(move |_| {
        if let Some(canvas) = multi_plot_ref.get() {
            multiplot(&canvas);
        }
    });

    // Return the view
    view! {
        <div>
            <canvas width="1500" height="700" _ref=multi_plot_ref></canvas>
        </div>
    }
}

#[component]
pub fn SimpleBarChart() -> impl IntoView {
    // Example data: Sales over a week (7 days)
    let sales_data = vec![100, 150, 130, 90, 110, 170, 140];

    // Reference to the canvas element
    let plot_ref = create_node_ref::<Canvas>();

    // Plot the chart on the canvas
    let plot_chart = move |plot_ref: &HtmlElement<Canvas>, data: &[i32]| {
        // Convert the plot_ref to HtmlCanvasElement explicitly
        let canvas: HtmlCanvasElement = plot_ref
            .unchecked_ref::<web_sys::HtmlCanvasElement>()
            .clone();

        // Set up the canvas backend
        let backend = CanvasBackend::with_canvas_object(canvas)
            .expect("Failed to initialize the canvas backend")
            .into_drawing_area();

        // Clear the canvas
        backend.fill(&WHITE).expect("Failed to clear the canvas");

        // Create the chart
        let mut chart = ChartBuilder::on(&backend)
            .caption("Simple Bar Chart", ("sans-serif", 20).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..7, 0..200) // Use i32 for the range
            .expect("Failed to build the chart");

        // Draw the X and Y axis
        chart
            .configure_mesh()
            .draw()
            .expect("Failed to draw the mesh");

        // Draw the bar series
        chart
            .draw_series(data.iter().enumerate().map(|(i, &value)| {
                Rectangle::new(
                    [(i as i32, 0), (i as i32 + 1, value)], // Convert i to i32
                    RED.filled(),
                )
            }))
            .expect("Failed to draw the bar chart");

        backend.present().expect("Failed to present the chart");
    };

    // Effect that triggers chart plotting when the component is mounted
    create_effect(move |_| {
        if let Some(canvas) = plot_ref.get() {
            plot_chart(&canvas, &sales_data);
        }
    });

    // Return the view
    view! {
        <div>
            <h2>"Sales Data for the Week"</h2>
            <canvas width="1800" height="700" _ref=plot_ref></canvas>
        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <MultiplePlot/>
            // <SimpleBarChart/>
        }
    })
}
