use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_quote};

pub fn shape(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = match find_wrapped(&input) {
        Ok(field) => field,
        Err(err) => return err.to_compile_error(),
    };
    let wrapped_id = &wrapped.ident;
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[
            parse_quote!(EuclideanCoordinate: Coordinate),
            parse_quote!(const EuclideanDim: usize),
        ],
        &[parse_quote!(#wrapped_ty: Shape<EuclideanCoordinate, EuclideanDim>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Shape<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
        #where_clause
        {
            fn distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
            where EuclideanCoordinate: num_traits::Float {
                self. #wrapped_id .distance(point)
            }

            fn signed_distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
            where EuclideanCoordinate: num_traits::Float {
                self. #wrapped_id .signed_distance(point)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::shape;
    use googletest::prelude::*;
    use quote::quote;
    use syn::parse_quote;

    #[googletest::test]
    fn shape_works_with_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello<S, T, const D: usize>
            where
                S: Shape<[T; D]>,
                T: Coordinate,
            {
                #[wrap]
                generator: S,
            }
        };

        let expected = quote! {
            impl<S, T, const D: usize, EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Shape<EuclideanCoordinate, EuclideanDim> for Hello<S, T, D>
            where
                S: Shape<[T; D]>,
                T: Coordinate,
                S: Shape<EuclideanCoordinate, EuclideanDim>
            {
                fn distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
                where EuclideanCoordinate: num_traits::Float {
                    self.generator.distance(point)
                }

                fn signed_distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
                where EuclideanCoordinate: num_traits::Float {
                    self.generator.signed_distance(point)
                }
            }
        };
        let output = shape(input);
        expect_that!(output.to_string(), eq(&expected.to_string()));
    }

    #[googletest::test]
    fn shape_without_wrap_attribute_emits_compile_error() {
        let input = parse_quote! {
            pub struct MissingWrap<S> {
                generator: S,
            }
        };
        let output = shape(input).to_string();
        expect_that!(output, contains_substring("compile_error"));
    }

    #[googletest::test]
    fn shape_with_duplicate_wrap_attributes_emits_compile_error() {
        let input = parse_quote! {
            pub struct DuplicateWrap<S> {
                #[wrap]
                first: S,
                #[wrap]
                second: S,
            }
        };
        let output = shape(input).to_string();
        expect_that!(output, contains_substring("compile_error"));
    }
}
