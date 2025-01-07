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
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            let mut rng = rand::thread_rng();
            let mut stars: Vec<(f64, f64, f64, bool)> = (0..500)
                .map(|_| {
                    (
                        rng.gen_range(-width / 2.0..width / 2.0), // x
                        rng.gen_range(-height / 2.0..height / 2.0), // y
                        rng.gen_range(1.0..width),                 // z
                        rng.gen_bool(0.1),                        // warp-speed effect
                    )
                })
                .collect();

            let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
                Rc::new(RefCell::new(None));
            let animate_clone = animate.clone();

            *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                context.set_fill_style(&JsValue::from_str("black"));
                context.fill_rect(0.0, 0.0, width, height);

                stars.iter_mut().for_each(|star| {
                    let (x, y, z, warp) = *star;
                    let scale = 128.0 / z;
                    let px = x * scale + width / 2.0;
                    let py = y * scale + height / 2.0;

                    if px >= 0.0 && px < width && py >= 0.0 && py < height {
                        if warp {
                            // Draw warp-speed streaks along trajectory
                            let length = 20.0;
                            let dx = x / z * length; // x trajectory offset
                            let dy = y / z * length; // y trajectory offset

                            context.set_stroke_style(&JsValue::from_str("white"));
                            context.begin_path();
                            context.move_to(px, py);
                            context.line_to(px + dx, py + dy);
                            context.stroke();
                        } else {
                            // Draw normal star as dot
                            let size = (1.0 - z / width) * 2.0;
                            context.set_fill_style(&JsValue::from_str("white"));
                            context.begin_path();
                            context.arc(px, py, size, 0.0, std::f64::consts::PI * 2.0).unwrap();
                            context.fill();
                        }
                    }

                    // Update star position
                    if warp {
                        star.2 -= 5.0; // Faster speed for warp effect
                    } else {
                        star.2 -= 2.0; // Regular speed
                    }

                    // Reset star if it moves out of view
                    if star.2 <= 0.0 {
                        *star = (
                            rng.gen_range(-width / 2.0..width / 2.0),
                            rng.gen_range(-height / 2.0..height / 2.0),
                            width,
                            rng.gen_bool(0.1), // Randomly decide if it's warp-speed
                        );
                    }
                });

                context.set_fill_style(&JsValue::from_str("white"));
                context.set_font("bold 48px sans-serif");
                context.set_text_align("center");
                context.set_text_baseline("middle");
                context.fill_text("Hyperspace", width / 2.0, height / 2.0).unwrap();

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
