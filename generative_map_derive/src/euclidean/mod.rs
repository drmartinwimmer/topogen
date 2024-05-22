use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field};

pub fn euclidean(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    quote! {
        impl<W, T, const D: usize> Euclidean<T, D> for #name<W, T, D>
        where
            W: Euclidean<T, D>,
            T: Coordinate,
        {
        }
    }
}

fn find_wrapped(input: &DeriveInput) -> Option<&Field> {
    if let Data::Struct(s) = &input.data {
        for field in s.fields.iter() {
            if field.attrs.iter().any(|attr| attr.path().is_ident("wrap")) {
                return Some(field);
            }
        }
    }
    None
}

// fn find_where_clause(token: Span, input: &DeriveInput) -> Option<WhereClause> {

// }

#[cfg(test)]
mod tests {
    use super::euclidean;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn euclidean_works() {
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
            impl<W, T, const D: usize> Euclidean<T, D> for Hello<W, T, D>
            where
                W: Euclidean<T, D>,
                T: Coordinate,
            {
            }
        };
        let output = euclidean(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
