use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Spring {
    k: f32,
    mass: f32,
    damping: f32,
    value: f32,
    target_value: f32,
    velocity: f32,
}

impl Spring {
    fn new(k: f32, mass: f32, damping: f32, value: f32, target_value: f32) -> Self {
        Spring {
            k,
            mass,
            damping,
            value,
            target_value,
            velocity: 0.0,
        }
    }

    // Method to calculate the force exerted by the spring
    fn calculate_force(&self) -> f32 {
        let displacement = self.value - self.target_value;
        -self.k * displacement
    }

    // Method to calculate the acceleration of the mass
    fn calculate_acceleration(&self) -> f32 {
        let force = self.calculate_force();
        // F = ma, so a = F / m
        (force - self.damping * self.velocity) / self.mass
    }

    // Method to update the spring system over a small time step
    fn update(&mut self, dt: f32) {
        let acceleration = self.calculate_acceleration();
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;
    }
}



struct Model {
    x: f32,
    y: f32,
    x_spring: Spring,
    y_spring: Spring,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .build()
        .unwrap();

    Model {
        x: 0.0,
        y: 0.0,
        x_spring: Spring::new(10.0, 1.0, 0.9, 0.0, 0.0),
        y_spring: Spring::new(10.0, 1.0, 0.9, 0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match app.mouse.buttons.left() {
        nannou::state::mouse::ButtonPosition::Down(_) => {
            model.x_spring.target_value = app.mouse.x;
            model.y_spring.target_value = app.mouse.y;
        },
        _ => {}
    }
    model.x_spring.update(0.1);
    model.y_spring.update(0.1);
    model.x = model.x_spring.value;
    model.y = model.y_spring.value;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
