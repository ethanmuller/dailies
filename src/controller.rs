use nannou::prelude::*;
use gilrs::{Gilrs, Event, EventType};

fn main() {
    nannou::app(data).update(update).run();
}

pub struct Data {
    r: f32,
    g: f32,
    b: f32,
    x: f32,
    y: f32,
    gilrs: Gilrs,
}

fn data(app: &App) -> Data {
    let gilrs = Gilrs::new().unwrap();
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
        x: 0.0,
        y: 0.0,
        gilrs,
    }
}


fn update(app: &App, data: &mut Data, frame_update: Update) {
    while let Some(Event { id: _, event, time: _ }) = data.gilrs.next_event() {
        match event {
            EventType::ButtonPressed(button, _) => {
                println!("Button pressed: {:?}", button);
            }
            EventType::ButtonReleased(button, _) => {
                println!("Button released: {:?}", button);
            }
            EventType::AxisChanged(axis, value, _) => {
                match axis {
                    gilrs::Axis::RightStickX => {
                        data.x = app.window_rect().right() * value * 0.666;
                    },
                    gilrs::Axis::RightStickY => {
                        data.y = app.window_rect().top() * value * 0.666;
                    },
                    _ => {},
                }
                println!("Axis changed: {:?}", value);
            }
            _ => {}
        }
    }

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
        .x_y(data.x, data.y)
        .rgb(data.r, data.g, data.b);
    draw.to_frame(app, &frame).unwrap();
}

