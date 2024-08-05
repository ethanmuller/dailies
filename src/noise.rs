use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    noise: Perlin,
    time: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();

    Model {
        time: 0.0,
        noise: Perlin::new(),
    }
}


fn update(_app: &App, model: &mut Model, _frame_update: Update) {
    model.time += 0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win = app.window_rect();
    let cols = win.w() as usize;
    let rows = win.h() as usize;

    for i in 0..cols {
        for j in 0..rows {
            let x = map_range(i, 0, cols, win.left(), win.right());
            let y = map_range(j, 0, rows, win.bottom(), win.top());
            let noise_value = model.noise.get([x as f64 * 0.1, y as f64 * 0.1, model.time]);
            let gray = map_range(noise_value, -1.0, 1.0, 0.0, 1.0);
            draw.rect()
                .x_y(x, y)
                .w_h(1.0, 1.0)
                .color(rgb(gray, gray, gray));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

