// tap tempo
//
// Press space or click on window rhythmically to set a tempo.
// Press backspace or delete to clear tempo.

use core::time;
use std::time::Instant;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

// Visual feedback displays current state:
// - no tempo set
// - initial tap
// - recording taps
// - tempo set
// Note that a single tap should set the down beat, not set tempo.
// Only on the second tap do we update to recording state.
//
// So there is a time cutoff that starts counting after the first tap,
// which should be based on a min BPM constant, but we'll see if we get there.....

#[derive(Debug)]
enum TapTempoState {
    NoTempoSet,
    InitialTap,
    RecordingTaps,
    TempoSet,
}

struct Model {
    state: TapTempoState,
    taps: Vec<Instant>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();


    Model {
        state: TapTempoState::NoTempoSet,
        taps: Vec::new(),
    }
}

fn calculate_bpm(model: &mut Model) -> Option<f64> {
    if model.taps.len() < 2 {
        return None;
    }

    let mut intervals = Vec::new();
    for i in 1..model.taps.len() {
        let duration = model.taps[i].duration_since(model.taps[i - 1]);
        intervals.push(duration.as_secs_f64());
    }

    let average_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
    Some(60.0 / average_interval)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let TapTempoState::RecordingTaps = model.state {
        check_tempo_set_cutoff(model);
    }
}

fn check_tempo_set_cutoff(model: &mut Model) {
    // check last tap and see if it was 2 seconds ago
    if let Some(last_tap) = model.taps.last() {
        let current_time = Instant::now();
        if current_time.duration_since(*last_tap).as_secs_f64() >= 2.0 {
            if let Some(bpm) = calculate_bpm(model) {
                model.state = TapTempoState::TempoSet;
                println!("bpm: {}", bpm);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.0, 0.0, 1.0);
    draw.to_frame(app, &frame).unwrap();
}

fn set_initial_time(model: &mut Model) {
    model.state = TapTempoState::InitialTap;
    model.taps.clear();
    println!("{:?}", model.state);
    model.taps.push(Instant::now());
}

fn set_additional_time(model: &mut Model) {
    model.taps.push(Instant::now());
    println!("tap");
}

fn tap(app: &App, model: &mut Model) {
    match model.state {
        TapTempoState::NoTempoSet => {
            set_initial_time(model)
        },
        TapTempoState::InitialTap => {
            model.state = TapTempoState::RecordingTaps;
            set_additional_time(model)
        },
        TapTempoState::RecordingTaps => {
            set_additional_time(model)
        },
        TapTempoState::TempoSet => {
            set_initial_time(model)
        },
    }
}

fn clear(app: &App, model: &mut Model) {
    model.state = TapTempoState::NoTempoSet;
    println!("{:?}", model.state)
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            tap(app, model)
        },
        Key::Back => {
            clear(app, model)
        },
        _ => {}
    }
}
fn mouse_pressed(app: &App, model: &mut Model, mouse_button: MouseButton) {
    match mouse_button {
        MouseButton::Left => {
            tap(app, model)
        },
        MouseButton::Right => {
            clear(app, model)
        },
        _ => {}
    }
        
}
