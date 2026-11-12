use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_quote};

pub fn bounded(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_id = &wrapped.ident;
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[
            parse_quote!(EuclideanCoordinate: Coordinate),
            parse_quote!(const EuclideanDim: usize),
        ],
        &[parse_quote!(#wrapped_ty: Bounded<EuclideanCoordinate, EuclideanDim>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Bounded<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
        #where_clause
        {
            fn within(&self, input: [EuclideanCoordinate; EuclideanDim]) -> bool {
                self. #wrapped_id .within(input)
            }

            fn bounding_box(&self) -> ::topogen::shapes::BoundingBox<EuclideanCoordinate, EuclideanDim> {
                self. #wrapped_id .bounding_box()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bounded;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn bounded_works_with_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello<S, T, const D: usize>
            where
                S: Bounded<[T; D]>,
                T: Coordinate,
            {
                #[wrap]
                generator: S,
            }
        };

        let expected = quote! {
            impl<S, T, const D: usize, EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Bounded<EuclideanCoordinate, EuclideanDim> for Hello<S, T, D>
            where
                S: Bounded<[T; D]>,
                T: Coordinate,
                S: Bounded<EuclideanCoordinate, EuclideanDim>
            {
                fn within(&self, input: [EuclideanCoordinate; EuclideanDim]) -> bool {
                    self.generator.within(input)
                }

                fn bounding_box(&self) -> ::topogen::shapes::BoundingBox<EuclideanCoordinate, EuclideanDim> {
                    self.generator.bounding_box()
                }
            }
        };
        let output = bounded(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
