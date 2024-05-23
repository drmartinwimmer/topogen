use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Field, GenericParam, Generics, WherePredicate};

pub fn euclidean(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[
            parse_quote!(EuclideanCoordinate: Coordinate),
            parse_quote!(const EuclideanDim: usize),
        ],
        &[parse_quote!(#wrapped_ty: Euclidean<EuclideanCoordinate, EuclideanDim>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Euclidean<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
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

fn create_forward_generics(input: &DeriveInput) -> Generics {
    let mut forward_generics = Generics::default();
    for param in input.generics.params.iter() {
        match param {
            syn::GenericParam::Type(ty) => {
                let ident = &ty.ident;
                forward_generics.params.push(parse_quote!(#ident))
            }
            syn::GenericParam::Const(ty) => {
                let ident = &ty.ident;
                forward_generics.params.push(parse_quote!(#ident))
            }
            syn::GenericParam::Lifetime(ty) => {
                let ident = &ty.lifetime;
                forward_generics.params.push(parse_quote!(#ident))
            }
        }
    }
    if !forward_generics.params.is_empty() {
        forward_generics.lt_token = Some(Default::default());
        forward_generics.gt_token = Some(Default::default());
    }
    forward_generics
}

fn expand_generics(
    input: &DeriveInput,
    generic_params: &[GenericParam],
    where_clauses: &[WherePredicate],
) -> Generics {
    let mut generics = input.generics.clone();
    for generic_param in generic_params {
        generics.params.push(generic_param.clone());
    }
    if !where_clauses.is_empty() {
        let where_clause = generics.make_where_clause();
        for clause in where_clauses {
            where_clause.predicates.push(clause.clone());
        }
    }
    generics
}

#[cfg(test)]
mod tests {
    use super::euclidean;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn euclidean_works_with_generics() {
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

    #[test]
    fn euclidean_works_without_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello {
                #[wrap]
                generator: Point<f64, 2>,
            }
        };

        let expected = quote! {
            impl<EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Euclidean<EuclideanCoordinate, EuclideanDim> for Hello
            where
                Point<f64, 2>: Euclidean<EuclideanCoordinate, EuclideanDim>
            {
            }
        };
        let output = euclidean(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
