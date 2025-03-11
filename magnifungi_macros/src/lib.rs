use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

use magnifungi_shared_types::entry_types::entry_enum::JsonEnum;


/// Macro that generates EntryEnumerator enums
#[proc_macro]
pub fn generate_enum_from_path(_: TokenStream) -> TokenStream {

    // Include the file content at compile time
    let json_str = include_str!("../../magnifungi_shared_types/enums.json");

    // Read it as a JsonEnum
    let enum_def: JsonEnum = serde_json::from_str(&json_str)
        .unwrap_or_else(|_| panic!("Invalid JSON format. => {:#?}", &json_str));

    let enum_name = syn::Ident::new(&enum_def.name, proc_macro2::Span::call_site());

    let variant_idents: Vec<syn::Ident> = enum_def
        .variants
        .iter()
        .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
        .collect();

    let match_arms =
        variant_idents
            .iter()
            .map(|ident| {
                let name = ident.to_string();
                quote! { #ident => #name }
            });

    let expanded = quote! {

        #[derive(EnumIter, Debug, PartialEq, Default, Clone)]
        pub enum #enum_name {
            #[default]
            #(#variant_idents,)*
        }

        impl IntoEntryEnumerator for #enum_name {
            fn to_str(&self) -> &str {
                match self {
                    #(Self::#match_arms,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

