use topology_traits_derive::TopologyTraits;

#[test]
fn test_derive_real_vector_space() {
    #[derive(TopologyTraits)]
    struct Point {
        x: u32,
    }
}
