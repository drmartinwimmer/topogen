use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_quote};

pub fn generator(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_id = &wrapped.ident;
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[parse_quote!(GeneratorInput)],
        &[parse_quote!(#wrapped_ty: Generator<GeneratorInput>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Generator<GeneratorInput> for #name #forward_generics
        #where_clause
        {
            type Output = #wrapped_ty ::Output;

            fn generate(&self, input: GeneratorInput) -> Self::Output {
                self. #wrapped_id .generate(input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::generator;
    use googletest::prelude::*;
    use quote::quote;
    use syn::parse_quote;

    #[googletest::test]
    fn generator_works_with_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello<G, T, const D: usize>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
            {
                #[wrap]
                generator: G,
            }
        };

        let expected = quote! {
            impl<G, T, const D: usize, GeneratorInput> Generator<GeneratorInput> for Hello<G, T, D>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
                G: Generator<GeneratorInput>
            {
                type Output = G::Output;

                fn generate(&self, input: GeneratorInput) -> Self::Output {
                    self.generator.generate(input)
                }
            }
        };
        let output = generator(input);
        expect_that!(output.to_string(), eq(&expected.to_string()));
    }
}
