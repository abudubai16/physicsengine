pub mod basics;
pub mod shapes;

pub mod simulation {
    use rand::{Rng, thread_rng};

    use super::basics::*;
    use super::shapes::*;

    use std::f32::consts::PI;

    use macroquad::prelude::{
        RED, WHITE, clear_background, draw_circle, get_frame_time, next_frame, screen_height,
        screen_width,
    };

    #[allow(dead_code)]
    fn build_frame() -> Vec<Rectangle> {
        let mut edges: Vec<Rectangle> = Vec::new();
        edges.push(Rectangle::new());
        edges
    }

    fn init_random() -> Circle {
        let mut rng = thread_rng();
        let radius = rng.gen_range(10.0..30.0);
        Circle {
            pos: Vector {
                x: rng.gen_range(radius..screen_width() - radius),
                y: rng.gen_range(radius..screen_height() - radius),
                z: 0.0,
            },
            vel: Vector {
                x: rng.gen_range(-10.0..10.0),
                y: rng.gen_range(-10.0..10.0),
                z: 0.0,
            },
            rot: ZERO_VEC,
            angular_vel: ZERO_VEC,
            radius: radius,
            color: RED,
            mass: PI * radius * radius,
        }
    }

    fn draw_circles(circles: &Vec<Circle>) {
        for circle in circles.iter() {
            draw_circle(circle.pos.x, circle.pos.y, circle.radius, circle.color());
        }
    }

    fn update_circles(circles: &mut Vec<Circle>) {
        for circle in circles.iter_mut() {
            circle.update_position();
            circle.update_rotation();
        }
    }

    #[allow(dead_code)]
    fn mtv() {
        // Minimum Translation Vector for collision resolution
    }

    #[macroquad::main("Physics Simulation")]
    pub async fn main() {
        let mut circles: Vec<Circle> = Vec::new();
        let g = 1.0;
        let dt = get_frame_time();

        println!("Value of g, frame time: {}, {}", g, dt);

        for _ in 0..20 {
            circles.push(init_random());
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
