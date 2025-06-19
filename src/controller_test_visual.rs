use nannou::prelude::*;
use gilrs::{Gilrs, Event, EventType};

fn main() {
    nannou::app(data).update(update).run();
}

pub struct Data {
    rx: f32,
    lx: f32,
    ry: f32,
    ly: f32,
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
        rx: 0.0,
        lx: 0.0,
        ry: 0.0,
        ly: 0.0,
        gilrs,
    }
}


fn update(app: &App, data: &mut Data, _frame_update: Update) {
    while let Some(Event { id: _, event, time: _ }) = data.gilrs.next_event() {
        match event {
            EventType::ButtonPressed(button, _) => {
                println!("Button pressed: {:?}", button);
            }
            EventType::ButtonReleased(button, _) => {
                println!("Button released: {:?}", button);
            }
            EventType::ButtonChanged(button, value, _) => {
                println!("button changed: {:?}: {}", button, value);
            }
            EventType::AxisChanged(axis, value, _) => {
                match axis {
                    gilrs::Axis::LeftStickX => {
                        data.lx = app.window_rect().right() * value;
                    },
                    gilrs::Axis::LeftStickY => {
                        data.ly = app.window_rect().top() * value;
                    },
                    gilrs::Axis::RightStickX => {
                        data.rx = app.window_rect().right() * value;
                    },
                    gilrs::Axis::RightStickY => {
                        data.ry = app.window_rect().top() * value;
                    },
                    _ => {},
                }
                println!("Axis changed: {}", value);
            }
            _ => {}
        }
    }
}

fn draw(app: &App, data: &Data, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.0, 0.0, 0.0);
    draw.ellipse()
        .x_y(data.lx, data.ly)
        .no_fill()
        .radius(66.666)
        .stroke(rgb(1.,1.,1.))
        .stroke_weight(3.0);
    draw.ellipse()
        .x_y(data.rx, data.ry)
        .radius(60.)
        .rgb(1.,1.,1.);
    draw.to_frame(app, &frame).unwrap();
}

