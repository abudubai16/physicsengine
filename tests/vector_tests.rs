use physics::basics::Vector;

mod vector {
    use super::Vector;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let result = v1 + v2;

        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector {
            x: 5.0,
            y: 7.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 2.0,
            y: 3.0,
            z: 0.0,
        };
        let result = v1 - v2;

        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let result = v1 * v2;
        println!("Dot product: {}", v2);
        assert_eq!(result, 11.0); // 1*3 + 2*4 = 3 + 8 = 11
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector {
            x: 2.0,
            y: 3.0,
            z: 0.0,
        };
        let scalar = 4.0;
        let result = v * scalar;

        assert_eq!(result.x, 8.0);
        assert_eq!(result.y, 12.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let result = v.magnitude();

        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let result = v.normalize();

        assert!((result.x - 0.6).abs() < 1e-6);
        assert!((result.y - 0.8).abs() < 1e-6);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let result = v1.cross_product(&v2);
        let expected = Vector {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_angle_to() {
        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let result = v1.angle_to(&v2);

        assert!((result - std::f32::consts::FRAC_PI_2).abs() < 1e-6); // 90 degrees in radians
    }
}
