pub mod constraints_tests {
    use basics::{Vector, ZERO_VEC};
    use physics::{
        basics::{self, Particle},
        contact, forces, shapes,
        world::{self, ParticleWorld},
    };
    use std::f32::consts;

    fn set_up_world() -> world::ParticleWorld {
        let mut world = world::ParticleWorld::new(800.0, 600.0, 10);

        let circle = shapes::ParticleCircle {
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
        world.particle_store.add_particle(Box::new(circle));
        world.particle_store.add_particle(Box::new(circle1));

        let p0_gravity = forces::GravityGenerator::new(Vector {
            x: 0.0,
            y: 9.81,
            z: 0.0,
        });
        world.force_registry.add(Box::new(p0_gravity), 0);

        world
    }

    fn distance_between_particles(world: &ParticleWorld) -> f32 {
        let pos1 = world.particle_store.get_particle(0).position();
        let pos2 = world.particle_store.get_particle(1).position();

        (pos1 - pos2).magnitude()
    }

    #[test]
    fn cable_constraint_test() {
        let mut world = set_up_world();

        // Add the cable constraint between the two particles
        let p1_constraint = contact::ParticleCable::new((0, 1), 50.0, 0.5);
        let p0_constraint = contact::ParticleCable::new((1, 0), 50.0, 0.5);
        world.constraints.push(Box::new(p1_constraint));
        world.constraints.push(Box::new(p0_constraint));

        let dt = 1.0 / 60.0;
        let mut time = 0.0;

        while time < 3.0 {
            world.start_frame();
            time += dt;
            world.run_physics(dt);
            world.contacts.clear();
            let distance = distance_between_particles(&world);
            assert!(
                (distance - 50.0).abs() <= 10.0,
                "Distance between particles: {}",
                distance
            );
        }
    }
}
