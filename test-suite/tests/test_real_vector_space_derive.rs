use derive_more::{Add, Mul};
use topology_traits_derive::Geodesic;

#[test]
fn test_derive_real_vector_space() {
    #[derive(Clone, Copy, Add, Mul, Geodesic)]
    struct Point {
        x: f64,
    }

    use topology_traits::Geodesic;
    let x = Point { x: 0.0 };
    let y = Point { x: 1.0 };
    assert_eq!(x.lerp(y, 0.5).x, 0.5);
}

// fn test_derive_real_vector_space_f32() {
//     #[derive(Clone, Copy, Add, Mul, Geodesic)]
//     #[topology_traits(scalars = "f32")]
//     struct Point {
//         x: f32,
//     }

//     use topology_traits::Geodesic;
//     let x = Point { x: 0.0f32 };
//     let y = Point { x: 1.0f32 };
//     assert_eq!(x.lerp(y, 0.5f32).x, 0.5f32);
// }

// fn test_derive_real_vector_space_f64_f32() {
//     #[derive(Clone, Copy, Add, Mul, Geodesic)]
//     #[topology_traits(scalars = "f64, f32")]
//     struct Point {
//         x: f64,
//     }

//     use topology_traits::Geodesic;
//     let x = Point { x: 0.0 };
//     let y = Point { x: 1.0 };
//     assert_eq!(x.lerp(y, 0.5f32).x, 0.5);
//     assert_eq!(x.lerp(y, 0.5).x, 0.5);
// }
