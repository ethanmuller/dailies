use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    agents: Vec<Agent>,
    agent_alpha: f32,
}


fn model(app: &App) -> Model {
    app.new_window()
        .size(1920, 1080)
        .view(view)
        .title("🔴")
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let agent_count = 30;
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect()))
        .collect();

    Model {
        agents,
        agent_alpha: 1.0,
    }
}

struct Agent {
    location: Vec2,
    velocity: Vec2,
    step_size: f32,
}

impl Agent {
    fn new(win_rect: Rect) -> Self {
        let location = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
        );
        let velocity = vec2(random_range(-0.333, 0.333), random_range(-0.333, 0.333));
        Agent {
            location,
            velocity,
            step_size: 6.0,
        }
    }

    fn wrap_around_edges(&mut self, bounds: Rect) {
        if self.location.x < bounds.left() - self.step_size {
            self.location.x = bounds.right() + self.step_size;
        }
        if self.location.x > bounds.right() + self.step_size {
            self.location.x = bounds.left() - self.step_size;
        }
        if self.location.y < bounds.bottom() - self.step_size {
            self.location.y = bounds.top() + self.step_size;
        }
        if self.location.y > bounds.top() + self.step_size {
            self.location.y = bounds.bottom() - self.step_size;
        }
    }

    fn update(&mut self, bounds: Rect) {
        // move
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;

        self.wrap_around_edges(bounds);
    }

    fn draw(&self, draw: &Draw, agent_alpha: f32) {
        let r = 1.0;
        let g = 1.0;
        let b = 1.0;

        draw.ellipse()
            .x_y(self.location.x, self.location.y)
            .radius(self.step_size/2.0)
            .rgba(r, g, b, agent_alpha);
    }

}

fn update(app: &App, model: &mut Model, _frame_update: Update) {
    let bounds = app.window_rect();

    for agent in &mut model.agents {
        agent.update(bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    let r = 0.0;
    let g = 0.0;
    let b = 0.0;

    let alpha = 1.0;

    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(r, g, b, alpha);

    model.agents.iter().for_each(|agent| {
        agent.draw(&draw, model.agent_alpha)
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
