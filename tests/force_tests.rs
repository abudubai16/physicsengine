mod forces {
    use basics::{Vector, ZERO_VEC};
    use physics::{basics, forces, shapes, world};
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
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            mass_inv: 1.0 / consts::PI,
            inertia_inv: 1.0 / (0.5 * consts::PI),
            force_accum: ZERO_VEC,
        };

        world.particle_store.add_particle(Box::new(circle));

        let p0_gravity = forces::GravityGenerator::new(Vector {
            x: 0.0,
            y: 9.81,
            z: 0.0,
        });
        world.force_registry.add(Box::new(p0_gravity), 0);

        world
    }

    #[test]
    fn test_gravity_force() {
        let mut world = set_up_world();

        let dt = 1.0;
        world.start_frame();
        world.run_physics(dt);
        let p0 = world.particle_store.get_particle(0);
        assert!(
            p0.velocity().y - 9.81 <= 0.01,
            "Velocity after gravity: {:?}",
            p0.velocity()
        );
    }
}
