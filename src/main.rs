use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use rand::Rng;

#[function_component(App)]
fn app() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with((), move |_| {
            // Access the canvas and its 2D context
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // Set canvas size based on window dimensions
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            // Initialize stars
            let mut rng = rand::thread_rng();
            let mut stars: Vec<(f64, f64, f64)> = (0..500)
                .map(|_| {
                    (
                        rng.gen_range(-width / 2.0..width / 2.0),
                        rng.gen_range(-height / 2.0..height / 2.0),
                        rng.gen_range(1.0..width),
                    )
                })
                .collect();

            // Animation loop
            let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
                Rc::new(RefCell::new(None));
            let animate_clone = animate.clone();

            *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                // Clear the canvas
                context.set_fill_style(&JsValue::from_str("black")); // Fixed: Removed `.expect()`
                context.fill_rect(0.0, 0.0, width, height);

                // Draw stars
                stars.iter_mut().for_each(|star| {
                    let (x, y, z) = *star;
                    let scale = 128.0 / z;
                    let px = x * scale + width / 2.0;
                    let py = y * scale + height / 2.0;

                    if px >= 0.0 && px < width && py >= 0.0 && py < height {
                        let size = (1.0 - z / width) * 2.0;
                        context.set_fill_style(&JsValue::from_str("white")); // Fixed: Removed `.expect()`
                        context.begin_path();
                        context.arc(px, py, size, 0.0, std::f64::consts::PI * 2.0).unwrap();
                        context.fill();
                    }

                    // Move stars closer to the viewer
                    star.2 -= 2.0;
                    if star.2 <= 0.0 {
                        *star = (
                            rng.gen_range(-width / 2.0..width / 2.0),
                            rng.gen_range(-height / 2.0..height / 2.0),
                            width,
                        );
                    }
                });

                // Request the next frame
                web_sys::window()
                    .unwrap()
                    .request_animation_frame(
                        animate_clone
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .unwrap();
            }) as Box<dyn FnMut()>));

            // Start animation
            web_sys::window()
                .unwrap()
                .request_animation_frame(
                    animate.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                )
                .unwrap();

            || {}
        });
    }

    html! {
        <canvas ref={canvas_ref} style="width: 100vw; height: 100vh;"></canvas>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}