mod forces {
    use physics::basics::*;
    use physics::forces::*;
    use physics::shapes::Circle;

    #[test]
    fn test_gravity_force() {
        let gravity_gen = GravityGenerator {
            gravity: Vector {
                x: 0.0,
                y: -9.81,
                z: 0.0,
            },
        };
        let gravity_gen: ForceGeneratorEntry = Box::new(gravity_gen);
        let particle: ParticleEntry = Box::from(Circle::new());
        let mut registry = ForceRegistry::new();
        registry.add(gravity_gen, particle);
        registry.update_forces(1.0);
        let expected_velocity = Vector {
            x: 0.0,
            y: -9.81,
            z: 0.0,
        };
        let particle = registry.entries[0].1.as_mut();
        particle.integrate(1.0);

        println!(
            "Particle velocity after gravity force: {:?}",
            particle.velocity()
        );

        assert!(particle.velocity().y - expected_velocity.y < 0.1);
    }
}
