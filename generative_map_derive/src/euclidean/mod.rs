use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, GenericParam,
};

pub fn euclidean(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_ty = &wrapped.ty;
    let mut generics = input.generics.clone();
    generics
        .params
        .push(parse_quote!(EuclideanCoordinate: Coordinate));
    generics
        .params
        .push(parse_quote!(const EuclideanDim: usize));
    let where_clause = match input.generics.where_clause.clone() {
        Some(mut where_clause) => {
            where_clause
                .predicates
                .push(parse_quote!(#wrapped_ty: Euclidean<EuclideanCoordinate, EuclideanDim>));
            where_clause
        }
        None => parse_quote!(where #wrapped_ty: Euclidean<EuclideanCoordinate, EuclideanDim>),
    };
    let mut forward_generics: Punctuated<GenericParam, Comma> = Punctuated::new();
    for param in input.generics.params.iter() {
        match param {
            syn::GenericParam::Type(ty) => {
                let ident = &ty.ident;
                forward_generics.push(parse_quote!(#ident))
            }
            syn::GenericParam::Const(ty) => {
                let ident = &ty.ident;
                forward_generics.push(parse_quote!(#ident))
            }
            syn::GenericParam::Lifetime(ty) => {
                let ident = &ty.lifetime;
                forward_generics.push(parse_quote!(#ident))
            }
        }
    }
    quote! {
        impl #generics Euclidean<EuclideanCoordinate, EuclideanDim> for #name <#forward_generics>
        #where_clause
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
            impl<G, T, const D: usize, EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Euclidean<EuclideanCoordinate, EuclideanDim> for Hello<G, T, D>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
                G: Euclidean<EuclideanCoordinate, EuclideanDim>
            {
            }
        };
        let output = euclidean(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
