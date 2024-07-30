pub struct Spring {
    pub k: f32,
    pub mass: f32,
    pub damping: f32,
    pub value: f32,
    pub target_value: f32,
    pub velocity: f32,
}

impl Spring {
    pub fn new(k: f32, mass: f32, damping: f32, value: f32, target_value: f32) -> Self {
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
    pub fn update(&mut self, dt: f32) {
        let acceleration = self.calculate_acceleration();
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;
    }
}
