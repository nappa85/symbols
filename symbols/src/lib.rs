#![deny(warnings)]
#![deny(missing_docs)]

//! # Symbols
//! 
//! This is an utility to build a proc-macro that connects to a database, retrieves data from given table and populates an enum variants with primary keys values  
//! It also generates a method for every non-primary-key field, and, when there are multiple primary keys, a costructor for every possible subset of primary keys

use std::{collections::HashMap, env, fs, future::Future, io};

use heck::{ToSnakeCase, ToUpperCamelCase};

use itertools::Itertools;

use proc_macro2::{Ident, Literal, Span, TokenStream};

use quote::quote;

use sea_orm::{
    DatabaseConnection, EntityTrait, Iterable, ModelTrait, PrimaryKeyToColumn, QueryFilter, Value,
};

use serde::{de::DeserializeOwned, Serialize};

pub use symbols_models::EntityFilter;

use syn::{
    punctuated::Punctuated, token::Comma, Fields, ItemEnum, Lit, LitBool, Meta, NestedMeta, Variant,
};

/// Main function  
/// Given a database model (via generics), an enum item, a list of arguments and an async function to retrieve a database connection
/// it populates the enum using primary key(s) values.  
/// Only string-typed primary keys are supported.
/// 
/// When a single primary key is present, it simply generate an as_str method and a TryFrom<&str> implementation.  
/// When multiple primary keys are present, it generates a costructor for every possible subset of primary keys.
/// 
/// For every non-primary key field of a supported type, it generates a const method to retrieve it.
/// 
/// Replacements can be done on every string-typed field, even primary keys, and are done using annotated parameters.  
/// Two type of replacements are supported:
/// * basic: written in the form #[macro(field = "enum")] or #[macro(field(type = "enum"))], where we are telling to replace string values from `field` with variants from enum `enum`, variant names will be the CamelCase version of field value.
/// * advanced: written in the form #[macro(field(type = "bar", fn = "foo"))], where we are telling to replace string values from `field` with a call to method `foo` from struct/enum `bar`, method output is expected to be of type `bar`.
pub async fn symbols<M, F, Fut>(
    item: &mut ItemEnum,
    args: &[NestedMeta],
    get_conn: F,
) -> syn::Result<TokenStream>
where
    M: EntityTrait + EntityFilter + Default,
    M::Model: Serialize + DeserializeOwned,
    M::Column: PartialEq,
    F: Fn() -> Fut,
    Fut: Future<Output = syn::Result<DatabaseConnection>>,
{
    let name = &item.ident;
    let primary_keys = M::PrimaryKey::iter()
        .map(|k| k.into_column())
        .collect::<Vec<_>>();

    let mut constructors = HashMap::new();
    let mut methods = HashMap::new();

    let data = get_data::<M, _, _>(get_conn).await?;

    data.iter()
        .map(|v| {
            let mut key_s = vec![];

            // scan primary keys
            for k in &primary_keys {
                let val = v.get(*k);
                // only string values are accepted
                if let Value::String(Some(s)) = val {
                    key_s.push(s.to_upper_camel_case());

                    // if we have a single primary key, create a method as_str and a counter-trait-impl TryFrom<&str>
                    if primary_keys.len() == 1 {
                        let key = Ident::new(&s.to_upper_camel_case(), Span::call_site());
                        let v = Literal::string(s.as_str());

                        let (_, method, _) =
                            methods.entry(String::from("as_str")).or_insert_with(|| {
                                (
                                    quote! { &'static str },
                                    Punctuated::<_, Comma>::new(),
                                    false,
                                )
                            });
                        method.push(quote! {
                            #name::#key => #v
                        });

                        let (_, method, _) =
                            methods.entry(String::from("try_from")).or_insert_with(|| {
                                (quote! { () }, Punctuated::<_, Comma>::new(), false)
                            });
                        method.push(quote! {
                            #v => Ok(#name::#key)
                        });
                    }
                } else {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        format!("Unrecognized value type {:?}", val),
                    ));
                }
            }
            // push primary keys into enum variants
            let key_ident = Ident::new(&key_s.join("_"), Span::call_site());
            item.variants.push(Variant {
                attrs: vec![],
                ident: key_ident.clone(),
                fields: Fields::Unit,
                discriminant: None,
            });
            // generate constructors for every combination of primary keys
            if primary_keys.len() > 1 {
                for n in 1..=primary_keys.len() {
                    for combo in primary_keys.iter().enumerate().combinations(n) {
                        let cols = combo.iter().map(|(_, col)| **col).collect::<Vec<_>>();
                        let method = combo
                            .iter()
                            .map(|(_, col)| format!("{:?}", col).to_snake_case())
                            .collect::<Vec<_>>()
                            .join("_and_");
                        let key = combo
                            .iter()
                            .map(|(index, _)| key_s[*index].clone())
                            .collect::<Vec<_>>();
                        let (_, method) = constructors
                            .entry(method)
                            .or_insert_with(|| (cols, HashMap::new()));
                        let (_, idents) = method
                            .entry(key.join("_"))
                            .or_insert_with(|| (key, Punctuated::<_, Comma>::new()));
                        idents.push(quote! { #name::#key_ident });
                    }
                }
            }

            // create a method for every non-primary_key column
            for col in M::Column::iter() {
                let replace = get_replacement::<M>(col, args);

                // skip self-describing methods (would be an as_str clone)
                if primary_keys.len() == 1 && primary_keys.contains(&col) && replace.is_none() {
                    continue;
                }

                // keep only managed data types
                let (t, value) = match v.get(col) {
                    Value::Bool(b) => (
                        quote! { bool },
                        b.map(|b| {
                            let v = LitBool::new(b, Span::call_site());
                            quote! { #v }
                        }),
                    ),
                    Value::TinyInt(n) => (
                        quote! { i8 },
                        n.map(|n| {
                            let v = Literal::i8_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::SmallInt(n) => (
                        quote! { i16 },
                        n.map(|n| {
                            let v = Literal::i16_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::Int(n) => (
                        quote! { i32 },
                        n.map(|n| {
                            let v = Literal::i32_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::BigInt(n) => (
                        quote! { i64 },
                        n.map(|n| {
                            let v = Literal::i64_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::TinyUnsigned(n) => (
                        quote! { u8 },
                        n.map(|n| {
                            let v = Literal::u8_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::SmallUnsigned(n) => (
                        quote! { u16 },
                        n.map(|n| {
                            let v = Literal::u16_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::Unsigned(n) => (
                        quote! { u32 },
                        n.map(|n| {
                            let v = Literal::u32_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::BigUnsigned(n) => (
                        quote! { u64 },
                        n.map(|n| {
                            let v = Literal::u64_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::Float(n) => (
                        quote! { f32 },
                        n.map(|n| {
                            let v = Literal::f32_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::Double(n) => (
                        quote! { f64 },
                        n.map(|n| {
                            let v = Literal::f64_unsuffixed(n);
                            quote! { #v }
                        }),
                    ),
                    Value::String(s) => match replace {
                        Some(Replacement::Type(r)) => (
                            r.clone(),
                            s.map(|s| {
                                let ident = Ident::new(&s.to_upper_camel_case(), Span::call_site());
                                quote! { #r::#ident }
                            }),
                        ),
                        Some(Replacement::Fn(f, Some(r))) => (
                            r.clone(),
                            s.map(|s| {
                                let v = Literal::string(s.as_str());
                                quote! { #r::#f(#v) }
                            }),
                        ),
                        Some(Replacement::Fn(_, None)) => {
                            // teoretically we could accept only a function, but we won't know the return type
                            return Err(syn::Error::new(
                                Span::call_site(),
                                format!("Missing parameter type for field {:?}", col),
                            ));
                        }
                        _ => (
                            quote! { &'static str },
                            s.map(|s| {
                                let v = Literal::string(s.as_str());
                                quote! { #v }
                            }),
                        ),
                    },
                    // disable ChronoDateTime for now, it would only produce methods for created_at and updated_at fields
                    // Value::ChronoDateTime(dt) => (quote! { chrono::NaiveDateTime }, Lit::Verbatim(Literal)),
                    _ => continue,
                };
                let (_, method, option) = methods
                    .entry(format!("{:?}", col))
                    .or_insert_with(|| (t, Punctuated::<_, Comma>::new(), false));
                if let Some(v) = value {
                    method.push(quote! {
                        #name::#key_ident => #v
                    });
                } else {
                    *option = true;
                }
            }

            Ok(())
        })
        .collect::<syn::Result<()>>()?;

    // decorate constructors
    let constructors = constructors.into_iter().map(|(name, (cols, body))| {
        let is_full = cols.len() == primary_keys.len();
        let fn_name = Ident::new(&format!("get_by_{}", name), Span::call_site());
        let signature = cols
            .iter()
            .map(|col| {
                let field_name =
                    Ident::new(&format!("{:?}", col).to_snake_case(), Span::call_site());
                match get_replacement::<M>(*col, args) {
                    Some(Replacement::Type(r)) => quote! { #field_name: #r },
                    _ => quote! { #field_name: &str },
                }
            })
            .collect::<Punctuated<_, Comma>>();
        let m = cols
            .iter()
            .map(|col| {
                let field_name =
                    Ident::new(&format!("{:?}", col).to_snake_case(), Span::call_site());
                quote! { #field_name }
            })
            .collect::<Punctuated<_, Comma>>();
        let body = body
            .iter()
            .map(|(_, (values, array_body))| {
                let args = cols
                    .iter()
                    .enumerate()
                    .map(|(index, col)| match get_replacement::<M>(*col, args) {
                        Some(Replacement::Type(r)) => {
                            let ident =
                                Ident::new(&values[index].to_upper_camel_case(), Span::call_site());
                            quote! { #r::#ident }
                        }
                        _ => {
                            let v = Literal::string(values[index].as_str());
                            quote! { #v }
                        }
                    })
                    .collect::<Punctuated<_, Comma>>();
                if is_full {
                    quote! {
                        (#args,) => Some(#array_body)
                    }
                } else {
                    quote! {
                        (#args,) => &[#array_body]
                    }
                }
            })
            .collect::<Punctuated<_, Comma>>();
        if is_full {
            quote! {
                pub const fn #fn_name(#signature) -> Option<Self> {
                    match (#m,) {
                        #body,
                        _ => None,
                    }
                }
            }
        } else {
            quote! {
                pub const fn #fn_name(#signature) -> &'static [Self] {
                    match (#m,) {
                        #body,
                        _ => &[],
                    }
                }
            }
        }
    });

    // separate try_from from other methods
    let try_from = methods
        .remove("try_from")
        .map(|(_, matches, _)| {
            quote! {
                impl<'a> TryFrom<&'a str> for #name {
                    type Error = String;
                    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                        match s {
                            #matches,
                            _ => Err(format!("Unknown {} {}", stringify!(#name), s)),
                        }
                    }
                }
            }
        })
        .unwrap_or_default();

    // decorate methods
    let methods: TokenStream = methods
        .into_iter()
        .filter_map(|(name, (t, matches, option))| {
            let n = Ident::new(&name.to_snake_case(), Span::call_site());
            if option {
                if matches.is_empty() {
                    Some(quote! {
                        pub const fn #n(&self) -> Option<#t> {
                            None
                        }
                    })
                } else {
                    Some(quote! {
                        pub const fn #n(&self) -> Option<#t> {
                            Some(match self {
                                #matches,
                                _ => return None,
                            })
                        }
                    })
                }
            } else {
                Some(quote! {
                    pub const fn #n(&self) -> #t {
                        match self {
                            #matches,
                        }
                    }
                })
            }
        })
        .chain(constructors)
        .collect();

    // output result
    Ok(quote! {
        #item

        impl #name {
            #methods
        }

        #try_from
    })
}

/// Replacement types
enum Replacement {
    Type(TokenStream),
    Fn(TokenStream, Option<TokenStream>),
}

/// Field replacement facility
/// Searches between macro arguments
fn get_replacement<M>(col: M::Column, args: &[NestedMeta]) -> Option<Replacement>
where
    M: EntityTrait,
    M::Column: PartialEq,
{
    let col_name = format!("{:?}", col);
    let field_name = col_name.to_snake_case();
    // search for replacements
    args.iter().find_map(|arg| {
        // simple #[macro(field = "enum")]
        if let NestedMeta::Meta(Meta::NameValue(mv)) = arg {
            if mv.path.is_ident(&col_name) || mv.path.is_ident(&field_name) {
                if let Lit::Str(s) = &mv.lit {
                    let ident = Ident::new(&s.value(), Span::call_site());
                    return Some(Replacement::Type(quote! { #ident }));
                }
            }
        }
        // quite complex #[macro(field(type = "enum", fn = "foo"))]
        if let NestedMeta::Meta(Meta::List(ml)) = arg {
            if ml.path.is_ident(&col_name) || ml.path.is_ident(&field_name) {
                return ml.nested.iter().fold(None, |mut acc, nested| {
                    if let NestedMeta::Meta(Meta::NameValue(mv)) = nested {
                        if let Lit::Str(s) = &mv.lit {
                            let ident = Ident::new(&s.value(), Span::call_site());
                            if mv.path.is_ident("type") {
                                if let Some(Replacement::Fn(f, None)) = acc {
                                    acc = Some(Replacement::Fn(f, Some(quote! { #ident })));
                                } else {
                                    acc = Some(Replacement::Type(quote! { #ident }));
                                }
                            } else if mv.path.is_ident("fn") {
                                if let Some(Replacement::Type(t)) = acc {
                                    acc = Some(Replacement::Fn(quote! { #ident }, Some(t)));
                                } else {
                                    acc = Some(Replacement::Fn(quote! { #ident }, None));
                                }
                            }
                        }
                    }
                    acc
                });
            }
        }
        None
    })
}

/// Data retrieve function with cache capabilities
/// File access is sync to not have to depend on an async runtime
async fn get_data<M, F, Fut>(get_conn: F) -> syn::Result<Vec<M::Model>>
where
    M: EntityTrait + EntityFilter + Default,
    M::Model: Serialize + DeserializeOwned,
    F: Fn() -> Fut,
    Fut: Future<Output = syn::Result<DatabaseConnection>>,
{
    let instance = M::default();
    let mut cache = env::current_dir().map_err(|e| {
        syn::Error::new(
            Span::call_site(),
            format!("Error retrieving current directory: {}", e),
        )
    })?;
    cache.push(instance.table_name());
    cache.set_extension("cache");
    if cache.exists() {
        println!("Using already existing cache file {}", cache.display());

        let file = fs::File::open(&cache).map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("Error reading {}: {}", cache.display(), e),
            )
        })?;
        bincode::deserialize_from(io::BufReader::new(file)).map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("Error deserializing {}: {}", cache.display(), e),
            )
        })
    } else {
        println!("Creating new cache file {}", cache.display());

        let conn = get_conn().await?;
        let data = M::find()
            .filter(M::filter())
            .all(&conn)
            .await
            .map_err(|e| syn::Error::new(Span::call_site(), e))?;
        let buf = bincode::serialize(&data).map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("Error serializing {}: {}", cache.display(), e),
            )
        })?;
        fs::write(&cache, buf).map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("Error writing {}: {}", cache.display(), e),
            )
        })?;
        Ok(data)
    }
}
