use nannou::prelude::*;

const EASE: f32 = 0.1;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .mouse_pressed(click)
        .build()
        .unwrap();

    Model {
        x: 0.0,
        y: 0.0,
        target_x: 0.0,
        target_y: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dx = model.x - model.target_x;
    let dx = EASE * dx;

    let dy = model.y - model.target_y;
    let dy = EASE * dy;

    model.y -= dy;
    model.x -= dx;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).x_y(model.x, model.y);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn click(app: &App, model: &mut Model, _mouse_button: MouseButton) {
    model.target_x = app.mouse.x;
    model.target_y = app.mouse.y;
}
