use std::time::Duration;

use proc_macro2::{Span, TokenStream};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use syn::{parse_macro_input, AttributeArgs, ItemEnum, Lit, Meta, NestedMeta};

use symbols::symbols;

mod model;

#[proc_macro_attribute]
pub fn example(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(input as ItemEnum);
    let args = parse_macro_input!(args as AttributeArgs);

    get_enum(&mut item, &args)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

async fn get_conn() -> syn::Result<DatabaseConnection> {
    let url = std::env::var("DATABASE_URL").map_err(|e| syn::Error::new(Span::call_site(), e))?;
    let mut options = ConnectOptions::new(url);
    options
        .min_connections(1)
        .max_connections(1)
        .connect_timeout(Duration::from_secs(1))
        .idle_timeout(Duration::from_secs(1));
    Database::connect(options)
        .await
        .map_err(|e| syn::Error::new(Span::call_site(), e))
}

fn get_enum(item: &mut ItemEnum, args: &[NestedMeta]) -> syn::Result<TokenStream> {
    // search for table, simply #[macro(table = "table_name")]
    let table = args
        .iter()
        .find_map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(mv)) = arg {
                if mv.path.is_ident("table") {
                    if let Lit::Str(s) = &mv.lit {
                        return Some(s.value());
                    }
                }
            }
            None
        })
        .ok_or_else(|| syn::Error::new(Span::call_site(), "Missing table attribute"))?;

    // start an async runtime to be able to use sea-orm
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            match table.as_str() {
                "best_selling_video_games" => {
                    symbols::<model::Entity, _, _>(item, args, get_conn).await
                }
                _ => Err(syn::Error::new(
                    Span::call_site(),
                    format!("Unrecognized table \"{}\"", table),
                )),
            }
        })
}
