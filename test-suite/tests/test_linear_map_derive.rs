use topology_traits_derive::Geodesic;

#[test]
fn test_map_generic_scalar_derive() {
    #[derive(Clone, Copy, Geodesic)]
    #[topology_traits(linear_map("mix" : R))]
    struct Point<R> {
        x: R,
    }

    impl<R> Point<R>
    where
        R: ::num_traits::real::Real,
    {
        pub fn mix(self, rhs: Self, fac: R) -> Point<R> {
            Point {
                x: self.x * fac + rhs.x * (1.0 - fac),
            }
        }
    }

    use topology_traits::Geodesic;
    let x = Point { x: 0.0 };
    let y = Point { x: 1.0 };
    assert_eq!(x.lerp(y, 0.5).x, 0.5)
}

fn test_map_scalar_derive() {
    #[derive(Clone, Copy, Geodesic)]
    #[topology_traits(linear_map("mix" : f64))]
    struct Point {
        x: f64,
    }

    impl Point {
        pub fn mix(self, rhs: Self, fac: f64) -> Point {
            Point {
                x: self.x * fac + rhs.x * (1.0 - fac),
            }
        }
    }

    use topology_traits::Geodesic;
    let x = Point { x: 0.0 };
    let y = Point { x: 1.0 };
    assert_eq!(x.lerp(y, 0.5).x, 0.5)
}
