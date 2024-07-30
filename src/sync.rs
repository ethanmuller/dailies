use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

mod tap;
use tap::{update as update_metro, view as view_metro, Model as Metro};

fn main() {
    nannou::app(model).update(update).run();
}

struct Agent {
    vector: Vec2,
    vector_old: Vec2,
    step_size: f32,
    angle: f32,
    noise_z: f64,
    win_rect: Rect,
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
            step_size: random_range(0.3, 6.0),
            angle: 0.0,
            noise_z: random_range(0.0, noise_z),
            win_rect,
        }
    }

    fn update(&mut self, speed: f32, noise_z_velocity: f64) {
        self.noise_z += noise_z_velocity;
        self.vector_old = self.vector;

        self.vector.x += self.angle.cos() * self.step_size * speed;
        self.vector.y += self.angle.sin() * self.step_size * speed;

        if self.vector.x < self.win_rect.left() - 10.0 {
            self.vector.x = self.win_rect.right() + 10.0;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.x > self.win_rect.right() + 10.0 {
            self.vector.x = self.win_rect.left() - 10.0;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.y < self.win_rect.bottom() - 10.0 {
            self.vector.y = self.win_rect.top() + 10.0;
            self.vector_old.y = self.vector.y;
        }
        if self.vector.y > self.win_rect.top() + 10.0 {
            self.vector.y = self.win_rect.bottom() - 10.0;
            self.vector_old.y = self.vector.y;
        }
    }

    fn update1(&mut self, noise: Perlin, z: f64, noise_scale: f64, noise_strength: f64) {
        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
            z,
        ]) * noise_strength;
        self.angle = n as f32;
    }

    fn update2(&mut self, noise: Perlin, noise_scale: f64, noise_strength: f64) {
        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
            self.noise_z,
        ]) * 24.0;
        self.angle = n as f32;
        self.angle = (self.angle - self.angle.floor()) * noise_strength as f32;
    }

    fn display(&self, model: &Model, draw: &Draw, stroke_weight: f32, agent_alpha: f32) {
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
            .stroke_weight(stroke_weight * self.step_size);
    }
}

pub struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
    noise_z_velocity: f64,
    agent_alpha: f32,
    stroke_width: f32,
    draw_mode: u8,
    noise_seed: u32,
    oscillator_freq_mult: f32,
    oscillator_amp: f32,
    start_time: std::time::Instant,
    oscillator: f32,
    oscillator_old: f32,
    oscillator_above_0: bool,
    oscillator_above_0_old: bool,
    metro: Metro,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .title("🔴")
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let noise_z_range = 0.4;
    let agent_count = 8192;
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect(), noise_z_range))
        .collect();

    let oscillator = 0.0;

    Model {
        agents,
        noise_scale: 30.0,
        noise_strength: 1.0,
        noise_z_velocity: 0.001,
        agent_alpha: 1.0,
        stroke_width: 0.5,
        oscillator_amp: 2.0,
        oscillator_freq_mult: 0.1,
        draw_mode: 1,
        noise_seed: 12,
        oscillator,
        oscillator_old: oscillator,
        oscillator_above_0: true,
        oscillator_above_0_old: true,
        start_time: std::time::Instant::now(),
        metro: Metro::default(),
    }
}

fn update(app: &App, model: &mut Model, frame_update: Update) {
    let noise = Perlin::new().set_seed(model.noise_seed);
    let elapsed = model.start_time.elapsed();
    let elapsed_secs = elapsed.as_secs_f32();
    let z = elapsed.as_secs_f64() * 0.5;

    update_metro(app, &mut model.metro, frame_update);

    model.oscillator_old = model.oscillator;
    model.oscillator_above_0_old = model.oscillator_above_0;

    if let Some(bpm) = model.metro.bpm {
        let frequency = bpm as f32 / 60.0 / 4.0;
        model.oscillator_freq_mult = frequency;
    }

    model.oscillator = pow(
        (elapsed_secs * std::f32::consts::PI * model.oscillator_freq_mult).sin(),
        3,
    ) * model.oscillator_amp;

    model.oscillator_above_0 = model.oscillator > 0.0;

    if model.oscillator_above_0 && model.oscillator_above_0 != model.oscillator_above_0_old {
        model.noise_seed = (random_f32() * 10000.0).floor() as u32;
    }

    for agent in &mut model.agents {
        match model.draw_mode {
            1 => agent.update1(noise, z, model.noise_scale, model.noise_strength),
            2 => agent.update2(noise, model.noise_scale, model.noise_strength),
            _ => (),
        }
        agent.update(model.oscillator, model.noise_z_velocity);
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

    draw.background().rgba(r, g, b, 1.0);

    model.agents.iter().for_each(|agent| {
        agent.display(model, &draw, model.stroke_width, model.agent_alpha);
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    view_metro(app, &model.metro, frame);
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

fn tap(model: &mut Model) {
    model.noise_seed = (random_f32() * 10000.0).floor() as u32;
    model.metro.tap()
}

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => tap(model),
        Key::Back => model.metro.clear(),
        Key::Delete => model.metro.clear(),
        Key::Q => app.quit(),
        Key::Z => {
            if let Some(bpm) = model.metro.bpm {
                model.metro.bpm = Some(bpm / 2.0);
            }
        }
        Key::X => {
            if let Some(bpm) = model.metro.bpm {
                model.metro.bpm = Some(bpm * 2.0);
            }
        }
        _ => {}
    }
}

pub fn mouse_pressed(_app: &App, model: &mut Model, mouse_button: MouseButton) {
    match mouse_button {
        MouseButton::Left => tap(model),
        MouseButton::Right => model.metro.clear(),
        _ => {}
    }
}
