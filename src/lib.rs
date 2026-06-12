use macroquad::prelude::*;

mod basics;
use basics::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Circle {
    pub position: Vector,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub radius: f32,
    pub color: Color,
}

impl Circle {
    fn init_random() -> Self {
        let radius = rand::gen_range(10.0, 30.0);
        Circle {
            position: Vector {
                x: rand::gen_range(radius, screen_width() - radius),
                y: rand::gen_range(radius, screen_height() - radius),
            },
            velocity: Velocity {
                x: rand::gen_range(-10.0, 10.0),
                y: rand::gen_range(-10.0, 10.0),
            },
            rotation: Rotation {
                angle: 0.0,
                angular_velocity: 0.0,
            },
            radius: radius,
            color: RED,
        }
    }

    fn check_collision(&self, other: &Circle) -> bool {
        if !self.simple_collision(other) {
            return false;
        }
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }
}

impl Shape for Circle {
    fn position(&self) -> Vector {
        self.position
    }

    fn velocity(&self) -> Velocity {
        self.velocity
    }

    fn rotation(&self) -> Rotation {
        self.rotation
    }

    fn color(&self) -> Color {
        self.color
    }

    fn new() -> Self {
        Circle::init_random()
    }

    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn update_rotation(&mut self) {
        self.rotation.angle += self.rotation.angular_velocity;
    }
}

impl BoundingBox for Circle {
    fn get_bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.position.x - self.radius,
            self.position.x + self.radius,
            self.position.y - self.radius,
            self.position.y + self.radius,
        )
    }
}

pub mod simulation {
    use super::Circle;
    use super::basics::*;
    use macroquad::prelude::*;

    fn draw_circles(circles: &Vec<Circle>) {
        for circle in circles.iter() {
            draw_circle(
                circle.position.x,
                circle.position.y,
                circle.radius,
                circle.color,
            );
        }
    }

    fn update_circles(circles: &mut Vec<Circle>) {
        for circle in circles.iter_mut() {
            circle.update_position();
            circle.update_rotation();
        }
    }

    fn mtv() {
        // Minimum Translation Vector for collision resolution
    }

    #[macroquad::main("Physics Simulation")]
    pub async fn main() {
        let mut circles: Vec<Circle> = Vec::new();
        let g = 1.0;
        let dt = get_frame_time();

        for _ in 0..20 {
            circles.push(Circle::init_random());
        }

        loop {
            clear_background(WHITE);

            // Draw circles
            draw_circles(&circles);

            // Update positions
            update_circles(&mut circles);

            // Force calculations and collision detection
            {}

            next_frame().await;
        }
    }
}
