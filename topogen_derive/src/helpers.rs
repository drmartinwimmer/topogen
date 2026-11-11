use syn::{parse_quote, Data, DeriveInput, Field, GenericParam, Generics, WherePredicate};

pub fn find_wrapped(input: &DeriveInput) -> Option<&Field> {
    if let Data::Struct(s) = &input.data {
        for field in s.fields.iter() {
            if field.attrs.iter().any(|attr| attr.path().is_ident("wrap")) {
                return Some(field);
            }
        }
    }
    None
}

pub fn create_forward_generics(input: &DeriveInput) -> Generics {
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

pub fn expand_generics(
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
