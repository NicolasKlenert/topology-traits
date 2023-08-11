# Topology-traits

Topological traits for generic mathematics in Rust.

This crate is supposed to be an extension to num-traits. That is, often one has to work with more than simple numbers, for example Points in 3D space. `topology-traits` aims to define traits describing element relationships in various mathematical spaces and properties of these spaces.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
topology-traits = "0.1"
```

At the moment this crate contains traits to describe the distance of two elements and the points lying between those two elements. Hereby we define
- distance to be the length of the shortest path between the elements and
- the points lying between the elements to be exactly the points on the shortest path.

For certain elements, calculating the shortest path between two elements may be costly, such the trait `Topology` allows to implement the calculation of the shortest path separately. Such algorithms are able to use this trait to gain performance if they need multiple properties between two elements.

## Crate Features

This crate can be used without the standard library (`#![no_std]`) by disabling the default `std` feature. Use this in Cargo.toml:
```toml
[dependencies.topology-traits]
version = "0.1"
default-features = false
features = ["libm"]
```

## Contributing

If you have a requirement for elements in mind which describes a mathematical property which is not numerical in nature and which is not covered by this crate yet, please tell us about it and create a new issue.

Furthermore all contributions are welcome, no matter how huge or tiny.

## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
