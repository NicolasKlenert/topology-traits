use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TopologyTraits)]
pub fn real_vector_space_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_real_vector_space(&ast)
}

// TODO: instead of Path = [Self; 2] use a newtype pattern
fn impl_real_vector_space(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ::topology_traits::Geodesic<f64> for #name {
            type Path = [Self; 2];
            fn shortest_path(self, to: Self) -> Self::Path {
                [self, to]
            }
        }

        impl<R: Real> ::topology_traits::Connected<#name, R> for [#name; 2]
        where
            #name: Mul<R, Output = #name>,
        {
            fn contract(&self, factor: R) -> #name {
                self[0] * (R::one() - factor) + self[1] * factor
            }
        }
    };
    gen.into()
}

//TODO: These Traits are usually not known -> we have to include them. Show at Serde how they are doing it...

//TODO: another macro would be a mapping to another trait implementing a mix -> this can be solved by using an attribute macro
//TODO: topology-traits(RealVectorSpace) and topology-traits(linear: NameOfTrait) and stuff like that!
