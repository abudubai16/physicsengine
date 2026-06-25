pub mod basics;
pub mod contact;
pub mod forces;
pub mod shapes;
pub mod world;

pub mod simulation {
    use crate::contact;
    use crate::forces;

    use super::basics::*;
    use super::shapes::*;
    use super::world;
    use std::f32::consts;

    use macroquad::prelude::{
        WHITE, clear_background, get_frame_time, next_frame, screen_height, screen_width,
    };

    #[macroquad::main("Physics Simulation")]
    pub async fn main() {
        let mut world = world::ParticleWorld::new(screen_width(), screen_height(), 10);

        let circle = ParticleCircle {
            pos: Vector {
                x: 100.0,
                y: 100.0,
                z: 0.0,
            },
            vel: Vector {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
            mass_inv: 1.0 / consts::PI,
            inertia_inv: 1.0 / (0.5 * consts::PI),
            force_accum: ZERO_VEC,
        };
        let mut circle1 = circle.clone();
        circle1.set_position(Vector {
            x: 150.0,
            y: 100.0,
            z: 0.0,
        });

        let p0_gravity = forces::GravityGenerator::new(Vector {
            x: 0.0,
            y: 9.81,
            z: 0.0,
        });

        let p1_constraint = contact::ParticleCable::new((0, 1), 50.0, 0.5);
        let p0_constraint = contact::ParticleCable::new((1, 0), 50.0, 0.5);

        world.particle_store.add_particle(Box::new(circle));
        world.particle_store.add_particle(Box::new(circle1));
        world.force_registry.add(Box::new(p0_gravity), 0);
        world.constraints.push(Box::new(p1_constraint));
        world.constraints.push(Box::new(p0_constraint));

        let dt = get_frame_time();

        println!("Starting simulation with dt = {}", dt);
        println!(
            "Screen dimensions: {}x{}",
            world.screen_width, world.screen_height
        );

        loop {
            world.start_frame();
            clear_background(WHITE);

            // Draw everything here
            world.draw(true);

            world.run_physics(dt);
            world.contacts.clear(); // Clear contacts for the next frame

            next_frame().await;
        }
    }
}
