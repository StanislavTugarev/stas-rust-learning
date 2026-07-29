mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            println!("Congratulations! Circle is created");
            Circle { radius }
        }

        pub fn new_1(radius: f32) -> Result<Self, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius should be positive"))
            }
        }

        pub fn new_2(radius: f32) -> Self {
            match radius {
                -10.0..=0.0 => panic!("radius is between -10.0 and 0.0"),
                ..=-10.0 => panic!("radius is lesser than -10.0"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

fn some_fn() {}

// to show an additional output of functions we need to use – cargo test --lib -- --show-output
// to run a specified test – cargo test --lib large_circle_should_contain_smaller
// to run a specified group of tests – cargo test --lib should_not
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_circle_should_contain_smaller() {
        some_fn(); // we can access a private function in a test
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(
            larger_circle.contains(&smaller_circle),
            true,
            "Custom failure message"
        );

        assert_ne!(larger_circle.contains(&smaller_circle), false);
        assert!(larger_circle.contains(&smaller_circle));
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(!smaller_circle.contains(&larger_circle), true);
    }

    #[test]
    fn should_not_create_a_circle() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(1.0)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "radius is lesser than -10.0")]
    fn should_not_create_and_panic() {
        let some_circle = shapes::Circle::new_2(-11.0);
    }

    // to run only ignored test we need to use – cargo test --lib -- --ignored
    #[test]
    #[ignore]
    fn huge_test() {
        // Code that runs for hours
    }
}
