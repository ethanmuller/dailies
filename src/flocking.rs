use nannou::prelude::*;

pub struct Model {
    agents: Vec<Agent>,
    agent_alpha: f32,
}

#[derive(Clone, PartialEq)]
struct Agent {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    perception: f32,
    max_speed: f32,
    max_steering: f32,
    radius: f32,
}

impl Agent {
    fn new(win_rect: Rect) -> Self {
        let position = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
        );
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        let radius = 3.0;
        Agent {
            position,
            acceleration: vec2(0.0, 0.0),
            perception: radius * 6.666,
            velocity,
            radius,
            max_speed: radius / 2.,
            max_steering: 0.07,
        }
    }

    fn update(&mut self, agents: &[Agent], bounds: Rect) {
        // move
        self.flock(agents);
        self.velocity = limit_magnitude(self.velocity, self.max_speed);
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.wrap_around_edges(bounds);
        self.acceleration *= 0.0;
    }

    fn flock(&mut self, agents: &[Agent]) {
        let alignment = self.align_velocities(agents) * 1.0;
        let cohesion = self.cohesion(agents) * 1.0;
        let separation = self.separation(agents) * 1.0;

        self.acceleration += alignment;
        self.acceleration += cohesion;
        self.acceleration += separation;
    }

    fn align_velocities(&mut self, agents: &[Agent]) -> nannou::geom::Vec2 {
        let mut total = 0;
        let mut steering = vec2(0.0, 0.0);
        for other in agents {
            let d = self.position.distance(other.position);
            if other != self && d < self.perception {
                steering += other.velocity;
                total += 1;
            }
        }
        if total > 0 {
            steering = steering / total as f32;
            steering = set_magnitude(steering, self.max_speed);
            steering = steering - self.velocity;
            steering = limit_magnitude(steering, self.max_steering);
        }

        return steering
    }

    fn cohesion(&mut self, agents: &[Agent]) -> nannou::geom::Vec2 {
        let mut total = 0;
        let mut steering = vec2(0.0, 0.0);
        for other in agents {
            let d = self.position.distance(other.position);
            if other != self && d < self.perception {
                steering += other.position;
                total += 1;
            }
        }
        if total > 0 {
            steering = steering / total as f32;
            steering = steering - self.position;
            steering = set_magnitude(steering, self.max_speed);
            steering = steering - self.velocity;
            steering = limit_magnitude(steering, self.max_steering);
        }

        return steering
    }

    fn separation(&mut self, agents: &[Agent]) -> nannou::geom::Vec2 {
        let mut total = 0;
        let mut steering = vec2(0.0, 0.0);
        for other in agents {
            let d = self.position.distance(other.position);
            if other != self && d < self.perception {
                let mut diff = self.position - other.position;
                diff = diff / d;
                steering += diff;
                total += 1;
            }
        }
        if total > 0 {
            steering = steering / total as f32;
            steering = set_magnitude(steering, self.max_speed);
            steering = steering - self.velocity;
            steering = limit_magnitude(steering, self.max_steering);
        }

        return steering
    }

    fn wrap_around_edges(&mut self, bounds: Rect) {
        if self.position.x < bounds.left() - self.velocity.x - self.radius*2.0 {
            self.position.x = bounds.right() + self.velocity.x + self.radius*2.0;
        }
        if self.position.x > bounds.right() + self.velocity.x + self.radius*2.0 {
            self.position.x = bounds.left() - self.velocity.x - self.radius*2.0;
        }
        if self.position.y < bounds.bottom() - self.velocity.y - self.radius*2.0 {
            self.position.y = bounds.top() + self.velocity.y + self.radius*2.0;
        }
        if self.position.y > bounds.top() + self.velocity.y + self.radius*2.0 {
            self.position.y = bounds.bottom() - self.velocity.y - self.radius*2.0;
        }
    }

    fn draw(&self, draw: &Draw, agent_alpha: f32) {
        let r = 1.0;
        let g = 1.0;
        let b = 1.0;

        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .radius(self.radius)
            .rgba(r, g, b, agent_alpha);
    }

}

fn update(app: &App, model: &mut Model, _frame_update: Update) {
    let bounds = app.window_rect();
    let agents_snapshot = model.agents.clone();

    for agent in &mut model.agents {
        agent.update(&agents_snapshot, bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    let r = 0.0;
    let g = 0.0;
    let b = 0.0;

    let alpha = 0.01;

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

fn model(app: &App) -> Model {
    app.new_window()
        .size(1920, 1080)
        .view(view)
        .title("🔴")
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let agent_count = 666;
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect()))
        .collect();

    Model {
        agents,
        agent_alpha: 1.0,
    }
}

fn main() {
    nannou::app(model).update(update).run();
}


fn limit_magnitude(v: Vec2, max_magnitude: f32) -> Vec2 {
    let current_magnitude = v.length();
    if current_magnitude > max_magnitude {
        v.normalize() * max_magnitude
    } else {
        v
    }
}

fn set_magnitude(v: Vec2, magnitude: f32) -> Vec2 {
    v.normalize() * magnitude
}
