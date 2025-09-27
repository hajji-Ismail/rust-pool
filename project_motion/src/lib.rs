#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}
impl Iterator for ThrowObject {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let dt = 1.0;
        let g = 9.8;

        // Update velocities
        let new_vx = self.actual_velocity.x;
        let new_vy = self.actual_velocity.y - g * dt;

        let new_x = self.actual_position.x + new_vx * dt; 
        let new_y = self.actual_position.y + self.actual_velocity.y * dt - 0.5 * g * dt * dt;

        // Update time
        let new_time = self.time + dt;

        let new_throw = ThrowObject {
            init_position: self.init_position.clone(),
            init_velocity: self.init_velocity.clone(),
            actual_position: Object { x: new_x, y: new_y },
            actual_velocity: Object { x: new_vx, y: new_vy },
            time: new_time,
        };

        // Update self to new state
        *self = new_throw.clone();

        Some(new_throw)
    }
}
