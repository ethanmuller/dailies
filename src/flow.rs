use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use gilrs::{Gilrs, Event, EventType, Button};

const DEADZONE: f32 = 0.125;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
    noise_z_velocity: f64,
    agent_alpha: f32,
    noise_seed: u32,
    start_time: std::time::Instant,
    oscillator: f32,
    oscillator_old: f32,
    oscillator_above_0: bool,
    oscillator_above_0_old: bool,
    frequency_multiplier: f32,
    gilrs: Gilrs,
    stick_y: f32,
}


fn model(app: &App) -> Model {
    let gilrs = Gilrs::new().unwrap();

    app.new_window()
        .size(1920, 1080)
        .view(view)
        .title("🔴")
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let noise_z_range = 0.4;
    let agent_count = 30;
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect(), noise_z_range))
        .collect();

    let oscillator = 0.0;

    Model {
        agents,
        noise_scale: 300.0,
        noise_strength: 0.0,
        noise_z_velocity: 0.01,
        agent_alpha: 1.0,
        noise_seed: 12,
        oscillator,
        oscillator_old: oscillator,
        oscillator_above_0: true,
        oscillator_above_0_old: true,
        frequency_multiplier: 1.0,
        start_time: std::time::Instant::now(),
        gilrs,
        stick_y: 0.0,
    }
}

struct Agent {
    vector: Vec2,
    vector_old: Vec2,
    step_size: f32,
    speed: f32,
    angle: f32,
    noise_z: f64,
}

impl Agent {
    fn new(win_rect: Rect, noise_z: f64) -> Self {
        let vector = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
        );
        Agent {
            vector,
            vector_old: vector,
            step_size: random_range(30.0, 60.0),
            speed: random_range(6.0, 10.0),
            angle: random_range(0.0, std::f32::consts::PI * 2.0),
            noise_z: random_range(0.0, noise_z),
        }
    }

    fn update(&mut self, oscillator: f32, noise_z_velocity: f64, bounds: Rect) {
        self.vector_old = self.vector;

        self.vector.x += self.angle.sin() * self.speed * oscillator;
        self.vector.y += self.angle.cos() * self.speed * oscillator;

        if self.vector.x < bounds.left() - self.step_size {
            self.vector.x = bounds.right() + self.step_size;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.x > bounds.right() + self.step_size {
            self.vector.x = bounds.left() - self.step_size;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.y < bounds.bottom() - self.step_size {
            self.vector.y = bounds.top() + self.step_size;
            self.vector_old.y = self.vector.y;
        }
        if self.vector.y > bounds.top() + self.step_size {
            self.vector.y = bounds.bottom() - self.step_size;
            self.vector_old.y = self.vector.y;
        }
    }

    fn apply_noise(&mut self, time: f32,  noise: Perlin, z: f64, noise_scale: f64, noise_strength: f64) {
        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale - time as f64,
            z,
        ]) * noise_strength;
        self.angle = n as f32;
    }

    fn display_trails(&self, model: &Model, draw: &Draw, agent_alpha: f32) {
        let elapsed = model.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs_f32();
        let r = (elapsed_secs * 0.06 * std::f32::consts::PI + 3.0)
            .sin()
            .abs();
        let g = (elapsed_secs * 0.01 * std::f32::consts::PI + 6.0)
            .sin()
            .abs();
        let b = (elapsed_secs * 0.1 * std::f32::consts::PI).sin().abs();

        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(r, g, b, agent_alpha)
            .stroke_weight(self.step_size/2.0);
        draw.ellipse()
            .x_y(self.vector.x, self.vector.y)
            .radius(self.step_size/2.0)
            .rgba(r, g, b, agent_alpha);
    }

    fn draw(&self, model: &Model, draw: &Draw, agent_alpha: f32) {
        let elapsed = model.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs_f32();
        let r = 1.0;
        let g = (elapsed_secs * 0.01 * std::f32::consts::PI + 6.0)
            .sin()
            .abs();
        let b = (elapsed_secs * 0.1 * std::f32::consts::PI).sin().abs();

        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(r, g, b, agent_alpha)
            .stroke_weight(self.step_size/15.0);
    }
}

fn update(app: &App, model: &mut Model, _frame_update: Update) {
    let noise = Perlin::new().set_seed(model.noise_seed);
    let elapsed = model.start_time.elapsed();
    let elapsed_secs = elapsed.as_secs_f32();
    let z = elapsed.as_secs_f64() * 0.5;

    let bounds = app.window_rect();

    model.oscillator_old = model.oscillator;
    model.oscillator_above_0_old = model.oscillator_above_0;
    model.oscillator = model.stick_y * 2.0;

    while let Some(Event { id: _, event, time: _ }) = model.gilrs.next_event() {
        match event {
            EventType::AxisChanged(axis, value, _) => {
                match axis {
                    gilrs::Axis::RightStickY => {
                        if value.abs() > DEADZONE {
                            model.stick_y = value;
                        } else {
                            model.stick_y = 0.0;
                        }
                    },
                    _ => {},
                }
            }

            EventType::ButtonPressed(button, _) => {
                match button {
                    Button::DPadUp => { model.frequency_multiplier = 4.0 },
                    Button::DPadRight => { model.frequency_multiplier = 2.0 },
                    Button::DPadDown => { model.frequency_multiplier = 1.0/4.0 },
                    Button::DPadLeft => { model.frequency_multiplier = 1.0/2.0 },

                    _ => {
                    },
                }
            },

            _ => {}
        }
    }

    // model.oscillator = model.stick_y;
     model.oscillator = model.oscillator + 0.1;

    model.oscillator_above_0 = model.oscillator > 0.0;

    if model.oscillator_above_0 && model.oscillator_above_0 != model.oscillator_above_0_old {
        model.noise_seed = (random_f32() * 10000.0).floor() as u32;
    }

    for agent in &mut model.agents {
        //agent.apply_noise(elapsed_secs, noise, z, model.noise_scale, model.noise_strength);
        agent.update(model.oscillator, model.noise_z_velocity, bounds);
    }


    let mut no_multiplier_buttons_pressed = true;

    for (_id, gamepad) in model.gilrs.gamepads() {
        no_multiplier_buttons_pressed = !gamepad.is_pressed(Button::DPadUp) &&
            !gamepad.is_pressed(Button::DPadRight) &&
            !gamepad.is_pressed(Button::DPadDown) &&
            !gamepad.is_pressed(Button::DPadLeft);
    }

    let no_multiplier_keys_pressed = !app.keys.down.contains(&Key::J) &&
        !app.keys.down.contains(&Key::K) &&
        !app.keys.down.contains(&Key::L) &&
        !app.keys.down.contains(&Key::Semicolon);

    if no_multiplier_keys_pressed && no_multiplier_buttons_pressed {
        model.frequency_multiplier = 1.0
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    let elapsed = model.start_time.elapsed();
    let elapsed_secs = elapsed.as_secs_f32();
    let r = (elapsed_secs * 0.03 * std::f32::consts::PI + 3.0)
        .sin()
        .abs()
        * 0.1;
    let g = (elapsed_secs * 0.005 * std::f32::consts::PI + 6.0)
        .sin()
        .abs()
        * 0.05;
    let b = (elapsed_secs * 0.05 * std::f32::consts::PI).sin().abs() * 0.3;

    let alpha = 0.03;

    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(r, g, b, alpha);

    model.agents.iter().for_each(|agent| {
        agent.display_trails(model, &draw, model.agent_alpha)
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

}

fn key_released(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let name = app.exe_name().unwrap() + ".png";
            println!("saving screenshot: {}", name);
            app.main_window().capture_frame(name);
        }
        _other_key => {}
    }
}

pub fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::Escape =>  app.quit(),
        Key::Q => app.quit(),
        _ => {}
    }
}
