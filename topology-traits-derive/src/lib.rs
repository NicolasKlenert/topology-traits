use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned, DeriveInput, Ident, ImplGenerics, LitStr, WhereClause};

#[derive(Debug)]
struct Settings {
    scalars: Option<Vec<Ident>>,
    mapping: Option<LitStr>,
}

#[proc_macro_derive(Geodesic, attributes(topology_traits))]
pub fn real_vector_space_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    let mut settings = Settings {
        scalars: None,
        mapping: None,
    };

    // for attr in ast.attrs {
    //     match &attr {
    //         Meta::List(list) if list.path.is_ident("hello") => {
    //             list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
    //                 .map_err(|_| {
    //                     // returning a specific syn::Error to teach the right usage of your macro
    //                     syn::Error::new(
    //                         list.span(),
    //                         // this indoc macro is just convenience and requires the indoc crate but can be done without it
    //                         indoc! {r#"
    //                             The `hello` attribute expects string literals to be comma separated

    //                             = help: use `#[hello("world1", "world2")]`
    //                         "#}
    //                     )
    //                 })?;
    //         }
    //         meta => {
    //             // returning a syn::Error would help with the compiler diagnostics and guide your macro users to get it right
    //             return Err(syn::Error::new(
    //                 meta.span(),
    //                 indoc! {r#"
    //                     The `hello` attribute is the only supported argument

    //                     = help: use `#[hello("world1")]`
    //                 "#})
    //             );
    //         }
    //     }
    // }

    eprintln!("INPUT: {:#?}", ast);

    // build settings
    for attr in &ast.attrs {
        if attr.path().is_ident("topology_traits") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("scalars") {
                    // here we collect all scalar ident
                    // TODO
                    return Ok(());
                }
                if meta.path.is_ident("linear_map") {
                    let value = meta.value()?;
                    let map: LitStr = value.parse().map_err(|_| {
                        syn::Error::new(meta.path.span(), "linear_map had no ident!")
                    })?;
                    settings.mapping = Some(map);
                    return Ok(());
                }
                Err(meta.error("unrecognized topology_traits"))
            })
            .unwrap();
        }
    }

    eprintln!("OUTPUT: {:#?}", settings);

    // let scalars = ast.attrs.iter().filter_map(|attr| {
    //     attr.meta.require_list().ok().and_then(|list| {
    //         if list.path == "topology_traits" {
    //             Some(list)
    //         } else {
    //             None
    //         }
    //     })
    // });

    // impl_trait(&ast, settings)
    if settings.mapping.is_none() {
        impl_real_vector_space(ast, settings)
    } else {
        impl_linear_mapping(ast, settings)
    }
}

fn impl_real_vector_space(mut ast: syn::DeriveInput, _: Settings) -> TokenStream {
    // CASE MATH:
    // We use addition and mulitplication
    // Here we can allow all real types "R" which allow addition and muliplication.
    // The Type and the bounds to it can be added to the other stuff. We do not need to check the object itself.

    let name: &Ident = &ast.ident;
    // let mut where_clause = ast.generics.make_where_clause();
    // where_clause.predicates.push(value)
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let function = quote!(self[0] * (R::one() - factor) + self[1] * factor);
    let mut appended_where_clause = if where_clause.is_none() {
        quote!(where)
    } else {
        quote!()
    };
    appended_where_clause.extend(quote!(
        #name: ::core::ops::Mul<R, Output = #name>,
        #name: ::core::ops::Add<Output = #name>,
        R: ::num_traits::real::Real,));

    // PROBLEM: for impl_generics, we need to add an __topology_traits_R
    // as implGenerics is sorted, we must instert is in the middle (between lifetimes and const generics)
    // such it is difficult to do and we must to the work beforehand?!
    // This creates more work than I though it would

    // https://docs.rs/syn/latest/syn/struct.Generics.html

    let gen = quote! {
        impl<R> ::topology_traits::Geodesic<R> for #name
        #where_clause #appended_where_clause
        {
            type Path = [Self; 2];
            fn shortest_path(self, to: Self) -> Self::Path {
                [self, to]
            }
        }

        impl<R> ::topology_traits::Connected<#name, R> for [#name; 2]
        #where_clause #appended_where_clause
        {
            fn contract(&self, factor: R) -> #name {
                #function
            }
        }
    };
    gen.into()
}

fn impl_linear_mapping(ast: syn::DeriveInput, settings: Settings) -> TokenStream {
    // CASE MAPPING:
    // Here we map to a function or a generic set of functions. We don't need any information on R.
    // However the function must take that R. Such we need the where clauses of this input argument
    // This might be on the function, this might however be also on the struct or trait.

    // OR: we could make it more stupid and just implement it for a set of types (instead of generics!)
    // AND/OR: allow for a list of the name of function, it's generics (impl_generics and ty_generics) and where clauses.
    // Just add everything to it (keep it simple and stupid)
    // This might be the best solution: start with types and go furhter after that
    // So each item needs:
    // - name and input_type (mandatory)
    // - impl_generics, ty_generics and where clauses

    // Add this options later... if necessary

    let name: &Ident = &ast.ident;
    // map error here!
    let fname: &Ident = &settings.mapping.unwrap().parse().unwrap();
    let function = quote!(self[0].#fname(self[1], factor));

    let gen = quote! {
        impl<R> ::topology_traits::Geodesic<R> for #name
        {
            type Path = [Self; 2];
            fn shortest_path(self, to: Self) -> Self::Path {
                [self, to]
            }
        }

        impl<R> ::topology_traits::Connected<#name, R> for [#name; 2]
        {
            fn contract(&self, factor: R) -> #name {
                #function
            }
        }
    };
    gen.into()
}

// TODO: instead of Path = [Self; 2] use a newtype pattern
// TODO: instead of f64 allow for multiple types -> look at attributes "scalars" for this!
// fn impl_trait(ast: &syn::DeriveInput, settings: Settings) -> TokenStream {
//     let name: &Ident = &ast.ident;
//     let function = match &settings.mapping {
//         None => quote!(self[0] * (R::one() - factor) + self[1] * factor),
//         Some(fname) => quote!(self[0].#fname(self[1], factor)),
//     };
//     let where_clause = match &settings.mapping {
//         None => quote!( where
//             #name: ::core::ops::Mul<R, Output = #name>,
//             #name: ::core::ops::Add<Output = #name>,
//             R: ::num_traits::real::Real,),
//         Some(_) => quote!(),
//     };

//     let gen = quote! {
//         impl<R> ::topology_traits::Geodesic<R> for #name
//         #where_clause
//         {
//             type Path = [Self; 2];
//             fn shortest_path(self, to: Self) -> Self::Path {
//                 [self, to]
//             }
//         }

//         impl<R> ::topology_traits::Connected<#name, R> for [#name; 2]
//         #where_clause
//         {
//             fn contract(&self, factor: R) -> #name {
//                 #function
//             }
//         }
//     };
//     gen.into()
// }

//TODO: These Traits are usually not known -> we have to include them. Show at Serde how they are doing it...

//TODO: another macro would be a mapping to another trait implementing a mix -> this can be solved by using an attribute macro
//TODO: topology-traits(RealVectorSpace) and topology-traits(linear: NameOfTrait) and stuff like that!
