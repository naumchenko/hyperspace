use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum StarEffect {
    Warp,
    Twinkle,
    Spiral,
}

#[derive(Properties, PartialEq)]
struct StarfieldProps {
    effect: StarEffect,
}

#[function_component(Starfield)]
fn starfield(props: &StarfieldProps) -> Html {
    let canvas_ref = use_node_ref();
    let effect = props.effect;

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

            match effect {
                StarEffect::Warp => animate_warp(context, width, height),
                StarEffect::Twinkle => animate_twinkle(context, width, height),
                StarEffect::Spiral => animate_spiral(context, width, height),
            }

            || {}
        });
    }

    html! {
        <canvas ref={canvas_ref} class="starfield-canvas" />
    }
}

fn animate_warp(context: CanvasRenderingContext2d, width: f64, height: f64) {
    let mut rng = rand::thread_rng();
    let mut stars: Vec<(f64, f64, f64, u8)> = (0..250)
        .map(|_| {
            (
                rng.gen_range(-width / 2.0..width / 2.0),
                rng.gen_range(-height / 2.0..height / 2.0),
                rng.gen_range(1.0..width),
                rng.gen_range(0..3),
            )
        })
        .collect();

    let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.set_fill_style_str("#0a0a0a");
        context.fill_rect(0.0, 0.0, width, height);

        stars.iter_mut().for_each(|star| {
            let (x, y, z, variant) = *star;
            let scale = 128.0 / z;
            let px = x * scale + width / 2.0;
            let py = y * scale + height / 2.0;
            let depth_factor = 1.0 - z / width;

            if px >= 0.0 && px < width && py >= 0.0 && py < height {
                let length = 30.0 * depth_factor;
                let dx = x / z * length;
                let dy = y / z * length;

                let gray = match variant {
                    0 => 180,
                    1 => 220,
                    _ => 150,
                };

                let alpha = 0.2 + depth_factor * 0.5;
                let streak_color = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha);
                context.set_stroke_style_str(&streak_color);
                context.set_line_width(0.5 + depth_factor * 1.5);
                context.begin_path();
                context.move_to(px, py);
                context.line_to(px + dx, py + dy);
                context.stroke();

                // Glow head
                let glow_alpha = alpha * 0.4;
                let glow_color = format!("rgba({}, {}, {}, {})", gray, gray, gray, glow_alpha);
                context.set_fill_style_str(&glow_color);
                context.begin_path();
                context.arc(px + dx, py + dy, 1.5 + depth_factor * 2.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.fill();
            }

            star.2 -= 3.0 + depth_factor * 4.0;

            if star.2 <= 0.0 {
                *star = (
                    rng.gen_range(-width / 2.0..width / 2.0),
                    rng.gen_range(-height / 2.0..height / 2.0),
                    width,
                    rng.gen_range(0..3),
                );
            }
        });

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
        .request_animation_frame(animate.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

fn animate_twinkle(context: CanvasRenderingContext2d, width: f64, height: f64) {
    let mut rng = rand::thread_rng();
    // x, y, base_size, phase, speed, base_gray
    let mut stars: Vec<(f64, f64, f64, f64, f64, f64)> = (0..350)
        .map(|_| {
            (
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                rng.gen_range(0.8..3.0),
                rng.gen_range(0.0..std::f64::consts::PI * 2.0),
                rng.gen_range(0.015..0.06),
                rng.gen_range(120.0..200.0), // Gray range
            )
        })
        .collect();

    let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.set_fill_style_str("#080808");
        context.fill_rect(0.0, 0.0, width, height);

        stars.iter_mut().for_each(|star| {
            let (x, y, base_size, phase, speed, base_gray) = *star;
            let brightness = (phase.sin() + 1.0) / 2.0;
            let size = base_size * (0.3 + brightness * 0.5);

            let gray = (base_gray + brightness * 60.0) as u32;
            let alpha = 0.3 + brightness * 0.5;
            let color = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha);
            context.set_fill_style_str(&color);

            context.begin_path();
            context.arc(x, y, size, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();

            // Multi-layer glow
            if brightness > 0.5 {
                // Inner glow
                let glow1 = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha * 0.2);
                context.set_fill_style_str(&glow1);
                context.begin_path();
                context.arc(x, y, size * 2.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.fill();

                // Outer glow
                let glow2 = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha * 0.1);
                context.set_fill_style_str(&glow2);
                context.begin_path();
                context.arc(x, y, size * 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.fill();
            }

            // Sparkle cross effect on brightest stars
            if brightness > 0.85 {
                let spark_alpha = (brightness - 0.85) * 4.0;
                let spark_color = format!("rgba(255, 255, 255, {})", spark_alpha * 0.5);
                context.set_stroke_style_str(&spark_color);
                context.set_line_width(0.5);
                let spike_len = size * 3.0;

                context.begin_path();
                context.move_to(x - spike_len, y);
                context.line_to(x + spike_len, y);
                context.move_to(x, y - spike_len);
                context.line_to(x, y + spike_len);
                context.stroke();
            }

            star.3 += speed;
        });

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
        .request_animation_frame(animate.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

fn animate_spiral(context: CanvasRenderingContext2d, width: f64, height: f64) {
    let mut rng = rand::thread_rng();
    // angle, distance, size, speed, arm (which spiral arm)
    let mut stars: Vec<(f64, f64, f64, f64, u8)> = (0..500)
        .map(|_| {
            let arm = rng.gen_range(0..4);
            let base_angle = (arm as f64) * std::f64::consts::PI / 2.0;
            let angle = base_angle + rng.gen_range(0.0..std::f64::consts::PI * 2.0);
            let distance = rng.gen_range(20.0..(width.min(height) / 2.0));
            (
                angle,
                distance,
                rng.gen_range(1.0..3.5),
                rng.gen_range(0.002..0.008),
                arm,
            )
        })
        .collect();

    let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let max_dist = width.min(height) / 2.0;
    let mut time: f64 = 0.0;

    *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.set_fill_style_str("#060606");
        context.fill_rect(0.0, 0.0, width, height);

        time += 0.005;

        // Draw subtle nebula clouds
        for i in 0..3 {
            let nebula_angle = time * 0.2 + (i as f64) * 2.0;
            let nebula_x = center_x + nebula_angle.cos() * 100.0;
            let nebula_y = center_y + nebula_angle.sin() * 80.0;
            let nebula_gray = 30 + (i as u32) * 5;
            let nebula_color = format!("rgba({}, {}, {}, 0.02)", nebula_gray, nebula_gray, nebula_gray);
            context.set_fill_style_str(&nebula_color);
            context.begin_path();
            context.arc(nebula_x, nebula_y, 150.0 + (i as f64) * 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();
        }

        stars.iter_mut().for_each(|star| {
            let (angle, distance, size, speed, arm) = *star;

            // Spiral distortion - stars further out lag behind
            let spiral_offset = distance / max_dist * 1.5;
            let display_angle = angle + spiral_offset;

            let x = center_x + display_angle.cos() * distance;
            let y = center_y + display_angle.sin() * distance;

            let dist_ratio = distance / max_dist;
            let base_gray = match arm {
                0 => 200,
                1 => 170,
                2 => 220,
                _ => 150,
            };
            let gray = base_gray - (dist_ratio * 50.0) as u32;

            // Main star
            let alpha = 0.4 + (1.0 - dist_ratio) * 0.4;
            let color = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha);
            context.set_fill_style_str(&color);
            context.begin_path();
            context.arc(x, y, size * 0.8, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();

            // Glow
            let glow_color = format!("rgba({}, {}, {}, {})", gray, gray, gray, alpha * 0.15);
            context.set_fill_style_str(&glow_color);
            context.begin_path();
            context.arc(x, y, size * 2.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();

            // Long flowing trail
            let trail_length = 8;
            for i in 1..=trail_length {
                let trail_angle = display_angle - speed * (i as f64) * 5.0;
                let trail_dist = distance + (i as f64) * 0.5;
                let trail_x = center_x + trail_angle.cos() * trail_dist;
                let trail_y = center_y + trail_angle.sin() * trail_dist;
                let trail_alpha = alpha * (0.3 - (i as f64) * 0.035);
                if trail_alpha > 0.0 {
                    let trail_color = format!("rgba({}, {}, {}, {})", gray, gray, gray, trail_alpha);
                    context.set_fill_style_str(&trail_color);
                    context.begin_path();
                    context.arc(trail_x, trail_y, size * (0.6 - (i as f64) * 0.04), 0.0, std::f64::consts::PI * 2.0).unwrap();
                    context.fill();
                }
            }

            star.0 += speed;
        });

        // Center glow - pulsing
        let pulse = (time * 2.0).sin() * 0.5 + 0.5;
        let core_glow = format!("rgba(150, 150, 150, {})", 0.03 + pulse * 0.03);
        context.set_fill_style_str(&core_glow);
        context.begin_path();
        context.arc(center_x, center_y, 60.0 + pulse * 15.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();

        let inner_core = format!("rgba(200, 200, 200, {})", 0.05 + pulse * 0.05);
        context.set_fill_style_str(&inner_core);
        context.begin_path();
        context.arc(center_x, center_y, 20.0 + pulse * 8.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();

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
        .request_animation_frame(animate.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

#[function_component(SignUpForm)]
fn sign_up_form() -> Html {
    let name = use_state(String::new);
    let email = use_state(String::new);
    let submitted = use_state(|| false);

    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_submit = {
        let submitted = submitted.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submitted.set(true);
        })
    };

    if *submitted {
        html! {
            <div class="form-container">
                <div class="success-message">
                    <h2>{"Welcome aboard!"}</h2>
                    <p>{"Thank you for signing up. Your journey begins now."}</p>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="form-container">
                <h2>{"Join the Journey"}</h2>
                <p class="form-subtitle">{"Sign up to explore the universe with us"}</p>
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <input
                            type="text"
                            placeholder="Your Name"
                            value={(*name).clone()}
                            oninput={on_name_input}
                            required=true
                        />
                    </div>
                    <div class="form-group">
                        <input
                            type="email"
                            placeholder="Your Email"
                            value={(*email).clone()}
                            oninput={on_email_input}
                            required=true
                        />
                    </div>
                    <button type="submit">{"Launch"}</button>
                </form>
            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <style>
                {r#"
                    *, *::before, *::after {
                        margin: 0;
                        padding: 0;
                        box-sizing: border-box;
                    }
                    html {
                        overflow: hidden;
                        height: 100%;
                    }
                    body {
                        height: 100%;
                        overflow: hidden;
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
                    }
                    .container {
                        height: 100%;
                        overflow-y: auto;
                        overflow-x: hidden;
                        scroll-snap-type: y mandatory;
                        scroll-behavior: smooth;
                        -ms-overflow-style: none;
                        scrollbar-width: none;
                    }
                    .container::-webkit-scrollbar {
                        display: none;
                    }
                    .page {
                        width: 100vw;
                        height: 100vh;
                        scroll-snap-align: start;
                        scroll-snap-stop: always;
                        flex-shrink: 0;
                        position: relative;
                    }
                    .starfield-canvas {
                        position: absolute;
                        top: 0;
                        left: 0;
                        display: block;
                        width: 100vw;
                        height: 100vh;
                        z-index: 1;
                    }
                    .banner {
                        position: absolute;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                        z-index: 2;
                        padding: 2rem;
                        text-align: center;
                        color: white;
                    }
                    .banner h1 {
                        font-size: clamp(2.5rem, 8vw, 6rem);
                        font-weight: 700;
                        margin-bottom: 1.5rem;
                        text-shadow: 0 0 40px rgba(0,0,0,0.8);
                        letter-spacing: -0.02em;
                    }
                    .banner p {
                        font-size: clamp(1rem, 2.5vw, 1.5rem);
                        max-width: 800px;
                        line-height: 1.8;
                        opacity: 0.9;
                        text-shadow: 0 0 20px rgba(0,0,0,0.8);
                    }
                    .banner-1 h1 {
                        background: linear-gradient(135deg, #00ffc8 0%, #00b4d8 50%, #0077b6 100%);
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    }
                    .banner-2 h1 {
                        background: linear-gradient(135deg, #00ff88 0%, #00d4aa 50%, #00b4d8 100%);
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    }
                    .scroll-indicator {
                        position: absolute;
                        bottom: 2rem;
                        left: 50%;
                        transform: translateX(-50%);
                        animation: bounce 2s infinite;
                        opacity: 0.7;
                        font-size: 2rem;
                    }
                    @keyframes bounce {
                        0%, 20%, 50%, 80%, 100% { transform: translateX(-50%) translateY(0); }
                        40% { transform: translateX(-50%) translateY(-10px); }
                        60% { transform: translateX(-50%) translateY(-5px); }
                    }
                    .form-container {
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%, -50%);
                        z-index: 2;
                        background: rgba(10, 20, 20, 0.7);
                        backdrop-filter: blur(20px);
                        border-radius: 20px;
                        padding: 3rem;
                        width: min(90%, 420px);
                        border: 1px solid rgba(0, 255, 200, 0.2);
                        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5), 0 0 40px -10px rgba(0, 255, 200, 0.1);
                    }
                    .form-container h2 {
                        color: white;
                        font-size: 2rem;
                        margin-bottom: 0.5rem;
                        text-align: center;
                    }
                    .form-subtitle {
                        color: rgba(255, 255, 255, 0.7);
                        text-align: center;
                        margin-bottom: 2rem;
                    }
                    .form-group {
                        margin-bottom: 1.25rem;
                    }
                    .form-group input {
                        width: 100%;
                        padding: 1rem 1.25rem;
                        border: none;
                        border-radius: 10px;
                        background: rgba(255, 255, 255, 0.15);
                        color: white;
                        font-size: 1rem;
                        outline: none;
                        transition: all 0.3s ease;
                    }
                    .form-group input::placeholder {
                        color: rgba(255, 255, 255, 0.5);
                    }
                    .form-group input:focus {
                        background: rgba(255, 255, 255, 0.25);
                        box-shadow: 0 0 0 3px rgba(0, 255, 200, 0.3);
                    }
                    button[type="submit"] {
                        width: 100%;
                        padding: 1rem;
                        border: none;
                        border-radius: 10px;
                        background: linear-gradient(135deg, #00ffc8 0%, #00b4d8 50%, #0077b6 100%);
                        color: #021a1a;
                        font-size: 1.1rem;
                        font-weight: 600;
                        cursor: pointer;
                        transition: all 0.3s ease;
                        margin-top: 0.5rem;
                        text-shadow: none;
                    }
                    button[type="submit"]:hover {
                        transform: translateY(-2px);
                        box-shadow: 0 10px 40px -10px rgba(0, 255, 200, 0.5);
                    }
                    .success-message {
                        text-align: center;
                        color: white;
                    }
                    .success-message h2 {
                        margin-bottom: 1rem;
                        color: #00ffc8;
                        text-shadow: 0 0 20px rgba(0, 255, 200, 0.5);
                    }
                "#}
            </style>
            <div class="page">
                <Starfield effect={StarEffect::Warp} />
                <div class="banner banner-1">
                    <h1>{"Explore the Cosmos"}</h1>
                    <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore."}</p>
                    <div class="scroll-indicator">{"↓"}</div>
                </div>
            </div>
            <div class="page">
                <Starfield effect={StarEffect::Twinkle} />
                <div class="banner banner-2">
                    <h1>{"Infinite Possibilities"}</h1>
                    <p>{"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta."}</p>
                    <div class="scroll-indicator">{"↓"}</div>
                </div>
            </div>
            <div class="page">
                <Starfield effect={StarEffect::Spiral} />
                <SignUpForm />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
