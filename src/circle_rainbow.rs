use nannou::prelude::*;

fn main() {
    nannou::app(data).update(update).run();
}

pub struct Data {
    r: f32,
    g: f32,
    b: f32,
}

fn data(app: &App) -> Data {
    app.new_window()
        .size(1920, 1080)
        .view(draw)
        .title("🔴")
        .build()
        .unwrap();


    Data {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    }
}


fn update(_app: &App, data: &mut Data, frame_update: Update) {
    let t = frame_update.since_start.as_secs_f32();
    let s = 0.333;
    data.r = ((t * PI * 3.0 * s).sin() + 1.0)/2.0;
    data.g = ((t * PI * 6.0 * s).sin() + 1.0)/2.0;
    data.b = ((t * PI * 1.0 * s).sin() + 1.0)/2.0;
}

fn draw(app: &App, data: &Data, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.0, 0.0, 0.0);
    draw.ellipse()
        .rgb(data.r, data.g, data.b);
    draw.to_frame(app, &frame).unwrap();
}

