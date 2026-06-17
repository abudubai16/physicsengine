mod forces {
    use physics::basics::*;
    use physics::forces::*;
    use physics::shapes::Circle;

    #[test]
    fn test_gravity_force() {
        let mut particle_store = ParticleStore::new();
        let particle = Box::new(Circle::new());
        particle_store.add_particle(particle);
        let force_gen = Box::new(GravityGenerator::new(Vector {
            x: 0.0,
            y: -9.81,
            z: 0.0,
        }));
        let mut force_registry = ForceRegistry::new();
        force_registry.add(force_gen, 0);
        let dt = 1.0;
        force_registry.update_forces(&mut particle_store, dt);

        let particle = particle_store.get_particle_mut(0).unwrap();
        particle.integrate(dt);
        let expected_vel = Vector {
            x: 0.0,
            y: -9.81,
            z: 0.0,
        };
        println!("Expected velocity: {:?}", particle.velocity());
        println!("Actual velocity: {:?}", expected_vel);
        assert!((particle.velocity() - expected_vel).y.abs() < 0.1);
    }
}
