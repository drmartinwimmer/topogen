use syn::{
    Data, DeriveInput, Error, Field, GenericParam, Generics, Result, WherePredicate, parse_quote,
};

pub fn find_wrapped(input: &DeriveInput) -> Result<&Field> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(Error::new_spanned(
            input,
            "wrapper derive macros can only be applied to structs",
        ));
    };
    let mut wrapped_fields = data_struct
        .fields
        .iter()
        .filter(|field| field.attrs.iter().any(|attr| attr.path().is_ident("wrap")));
    let Some(first) = wrapped_fields.next() else {
        return Err(Error::new_spanned(
            input,
            "expected exactly one field annotated with #[wrap]",
        ));
    };
    if let Some(duplicate) = wrapped_fields.next() {
        return Err(Error::new_spanned(
            duplicate,
            "at most one field may be annotated with #[wrap]",
        ));
    }
    Ok(first)
}

pub fn create_forward_generics(input: &DeriveInput) -> Generics {
    let mut forward_generics = Generics::default();
    for param in &input.generics.params {
        match param {
            syn::GenericParam::Type(ty) => {
                let ident = &ty.ident;
                forward_generics.params.push(parse_quote!(#ident));
            }
            syn::GenericParam::Const(ty) => {
                let ident = &ty.ident;
                forward_generics.params.push(parse_quote!(#ident));
            }
            syn::GenericParam::Lifetime(ty) => {
                let ident = &ty.lifetime;
                forward_generics.params.push(parse_quote!(#ident));
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
