use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, GenericParam, Ident, Item, Token, Visibility, WhereClause};

type DeriveResult<T> = Result<T, TokenStream2>;

macro_rules! error {
    ($message:expr $(,)?) => {
        quote! {
            compile_error!($message);
        }
    };
}

#[proc_macro_derive(Family)]
pub fn derive_family(input: TokenStream) -> TokenStream {
    derive_family_(input.into())
        .unwrap_or_else(std::convert::identity)
        .into()
}

fn derive_family_(input: TokenStream2) -> DeriveResult<TokenStream2> {
    let (ident, params, where_clause, vis) = parse_item(input)?;
    if where_clause.is_some() {
        return Err(error!("#[derive(Family)] does not support generic bounds"));
    }
    let mut params = strip_params(&params)?;
    if params.len() == 0 {
        return Err(error!(
            "#[derive(Family)] requires item with at least one generic parameter"
        ));
    }
    let mut tokens = TokenStream2::new();
    let mut prev_family_name;
    let mut family_name = ident;
    while !params.is_empty() {
        prev_family_name = family_name;
        family_name = format_ident!("{}Family", &prev_family_name);
        tokens.extend(decl_typefamily(
            &vis,
            &prev_family_name,
            &family_name,
            &params,
        ));
        params.pop().unwrap();
    }
    Ok(tokens.into())
}

/// Strip type parameters with bounds into simple ident-only type parameters.
fn strip_params(
    generic_params: &Punctuated<GenericParam, Token![,]>,
) -> DeriveResult<Punctuated<&Ident, Token![,]>> {
    let mut params = Punctuated::<&Ident, Token![,]>::new();
    for param in generic_params {
        let (ident, bounds, attrs) = match &param {
            GenericParam::Type(t) => (&t.ident, &t.bounds, &t.attrs),
            GenericParam::Lifetime(..) | GenericParam::Const(..) => {
                todo!("lifetime/const paramter")
            }
        };
        if !attrs.is_empty() {
            return Err(error!(
                "#[derive(Family)] does not support attributed parameters are not supported by "
            ));
        }
        if !bounds.is_empty() {
            return Err(error!("#[derive(Family)] does not support generic bounds"));
        }
        params.push_value(ident);
        params.push_punct(Token![,](Span::call_site()));
    }
    Ok(params)
}

fn parse_item(
    input: TokenStream2,
) -> DeriveResult<(
    Ident,
    Punctuated<GenericParam, Token![,]>,
    Option<WhereClause>,
    Visibility,
)> {
    let item = syn::parse2::<Item>(input).map_err(|_| error!("syn failed to parse this item"))?;
    match item {
        Item::Enum(item) => Ok((
            item.ident,
            item.generics.params,
            item.generics.where_clause,
            item.vis,
        )),
        Item::Struct(item) => Ok((
            item.ident,
            item.generics.params,
            item.generics.where_clause,
            item.vis,
        )),
        Item::Union(item) => Ok((
            item.ident,
            item.generics.params,
            item.generics.where_clause,
            item.vis,
        )),
        _ => unreachable!("{}:{}:{}", file!(), line!(), column!()),
    }
}

/// Declare one TypeFamily struct and impl Family for it.
fn decl_typefamily(
    vis: &Visibility,
    prev_family_name: &Ident,
    family_name: &Ident,
    params: &Punctuated<&Ident, Token![,]>,
) -> TokenStream2 {
    let mut prev_params = params.clone();
    prev_params.pop();
    prev_params.pop_punct();
    let new_param = params.last().unwrap();
    let struct_body = match prev_params.len() {
        0 => None,
        1 => Some(quote! { (std::marker::PhantomData<#prev_params>) }),
        _ => Some(quote! { (std::marker::PhantomData<(#prev_params)>) }),
    };
    quote! {
        #vis struct #family_name<#prev_params> #struct_body;
        impl<#prev_params> typefamilies::Family for #family_name<#prev_params> {
            type This<#new_param> = #prev_family_name<#params>;
        }
    }
}

// The following was my attempt at implementing HKT's with generic bounds.

// /// Merge type bounds in param with those in where clause into one where clause.
// fn merge_where_clause(
//     generic_params: &Punctuated<GenericParam, Token![,]>,
//     where_clause: Option<&WhereClause>,
// ) -> DeriveResult<Option<WhereClause>> {
//     let (mut predicates, where_token) = match where_clause {
//         Some(WhereClause {
//             where_token,
//             predicates,
//         }) => (predicates.clone(), Some(*where_token)),
//         None => (Punctuated::<WherePredicate, Token![,]>::new(), None),
//     };
//     for param in generic_params {
//         match &param {
//             GenericParam::Type(TypeParam {
//                 attrs,
//                 ident,
//                 colon_token,
//                 bounds,
//                 eq_token: _,
//                 default: _,
//             }) => {
//                 if !attrs.is_empty() {
//                     return Err(error!(
//                         "Attributed parameters are not supported by #[derive(Family)]"
//                     ));
//                 }
//                 if bounds.is_empty() {
//                     continue;
//                 }
//                 let predicate = PredicateType {
//                     lifetimes: None,
//                     bounded_ty: type_with_single_ident(ident.clone()),
//                     colon_token: colon_token
//                         .as_ref()
//                         .cloned()
//                         .unwrap_or_else(|| Token![:](Span::call_site())),
//                     bounds: bounds.clone(),
//                 };
//                 predicates.push(WherePredicate::Type(predicate));
//             }
//             GenericParam::Lifetime(..) | GenericParam::Const(..) => {
//                 todo!("lifetime/const paramter")
//             }
//         }
//     }
//     if predicates.is_empty() {
//         Ok(None)
//     } else {
//         Ok(Some(WhereClause {
//             where_token: where_token.unwrap_or_else(|| Token![where](Span::call_site())),
//             predicates,
//         }))
//     }
// }

// fn filter_where_clause(
//     where_clause: &WhereClause,
//     params: &Punctuated<&Ident, Token![,]>,
// ) -> WhereClause {
//     let mut result_predicates = Punctuated::<WherePredicate, Token![,]>::new();
//     for predicate in &where_clause.predicates {
//         match predicate {
//             WherePredicate::Lifetime(_) => todo!("Lifetime parameters"),
//             p @ WherePredicate::Type(PredicateType {
//                 lifetimes: _,
//                 bounded_ty,
//                 colon_token: _,
//                 bounds: _,
//             }) if type_as_single_ident(bounded_ty).is_some_and(|t| params_have(params, t)) => {
//                 result_predicates.push(p.clone());
//             }
//             _ => (),
//         }
//     }
//     WhereClause {
//         where_token: where_clause.where_token,
//         predicates: result_predicates,
//     }
// }

// /// If a list of generic params contains a type.
// fn params_have(params: &Punctuated<&Ident, Token![,]>, ident: &Ident) -> bool {
//     params.iter().find(|&&x| x == ident).is_some()
// }

// /// Make a type that consist with just a single ident.
// fn type_with_single_ident(ident: Ident) -> Type {
//     Type::Path(TypePath {
//         qself: None,
//         path: Path {
//             leading_colon: None,
//             segments: {
//                 let mut x = Punctuated::new();
//                 x.push_value(PathSegment {
//                     ident: ident.clone(),
//                     arguments: PathArguments::None,
//                 });
//                 x
//             },
//         },
//     })
// }

// /// If a type consists of a single ident, return the ident, returns `None` otherwise.
// fn type_as_single_ident(type_: &Type) -> Option<&Ident> {
//     match type_ {
//         Type::Path(TypePath {
//             qself: None,
//             path:
//                 Path {
//                     leading_colon: None,
//                     segments,
//                 },
//         }) => match segments.first() {
//             Some(PathSegment {
//                 ident,
//                 arguments: PathArguments::None,
//             }) => Some(ident),
//             _ => None,
//         },
//         _ => None,
//     }
// }
