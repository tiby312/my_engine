use gloo::console::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() {
    log!("demo start!");

    let canvas = shogo::utils::get_canvas_by_id("mycanvas");
    let ctx = shogo::utils::get_context_webgl2(&canvas);
    let button = shogo::utils::get_element_by_id("mybutton");
    let shutdown_button = shogo::utils::get_element_by_id("shutdownbutton");

    let mut engine = shogo::engine(60);

    let _handle = engine.add_mousemove(&canvas);
    let _handle = engine.add_click(&button);
    let _handle = engine.add_click(&shutdown_button);

    let mut mouse_pos = [0.0; 2];
    let mut color_iter = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ]
    .into_iter()
    .cycle();
    let mut current_color = color_iter.next().unwrap_throw();

    let mut gl_prog = shogo::points::create_draw_system(&ctx).unwrap_throw();

    let mut verts = Vec::new();
    'outer: loop {
        for res in engine.next().await.events {
            match res.event {
                shogo::Event::MouseClick(_mouse) => {
                    if res.element == button {
                        current_color = color_iter.next().unwrap_throw();
                    } else if res.element == shutdown_button {
                        break 'outer;
                    }
                }
                shogo::Event::MouseMove(mouse) => {
                    if res.element == *canvas.as_ref() {
                        mouse_pos = convert_coord(res.element, mouse);
                    }
                }
            }
        }

        verts.clear();
        verts.push(shogo::points::Vertex([
            mouse_pos[0] as f32,
            mouse_pos[1] as f32,
            0.0,
        ]));

        ctx.clear_color(0.13, 0.13, 0.13, 1.0);
        ctx.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl_prog(shogo::points::Args {
            context: &ctx,
            vertices: &verts,
            game_dim: [canvas.width() as f32, canvas.height() as f32],
            as_square: false,
            color: &current_color,
            offset: &[0.0, 0.0],
            point_size: 30.0,
        })
        .unwrap_throw();
    }

    log!("all done!");
}

fn convert_coord(canvas: web_sys::HtmlElement, e: web_sys::MouseEvent) -> [f64; 2] {
    let [x, y] = [e.client_x() as f64, e.client_y() as f64];
    let bb = canvas.get_bounding_client_rect();
    let tl = bb.x();
    let tr = bb.y();
    [x - tl, y - tr]
}
