use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};




/// Helper function to clean litteral strings into fine CamelCase Strings for enums
fn clean_for_enum(name: &str) -> String {
    use convert_case::Case;
    name.to_case(Case::Pascal)
}

/// Macro to generate all enum types from /magnifungi_shared_types/enums.json
#[proc_macro]
pub fn generate_enums_from_path(_: TokenStream) -> TokenStream {

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct JsonEnum {
        pub name: String,
        pub custom: bool,
        pub variants: Vec<String>,
    }

    let json_str = include_str!("../../magnifungi_shared_types/enums.json");

    let mut deser_enums: Vec<JsonEnum> =
        match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => panic!("Invalid JSON format. => {:#?}", e.to_string()),
        };
    
    let mut multiple_streams = Vec::new();

    deser_enums
        .iter_mut()
        .for_each(|this_enum| {

            let enum_name = syn::Ident::new(
                &clean_for_enum(&this_enum.name.replace("Name", "")),
                proc_macro2::Span::call_site()
            );

            let variants = this_enum
                .variants
                .iter();

            let variant_idents = variants
                .clone()
                .map(|v| syn::Ident::new(
                        &clean_for_enum(v),
                        proc_macro2::Span::call_site()
                ));

            let enum_string_name = proc_macro2::Literal::string(&this_enum.name);

            let variants_strings_match_arms = variants
                .zip(variant_idents.clone())
                .map(|(ident, clean_idents)| {
                    quote! { #clean_idents => #ident }
                });

            let null_custom_variants = if this_enum.custom {
                quote! {
                    Null,
                    Custom(String),  // Make sure `Custom` is included
                }
            } else {
                quote! { Null, }
            };

            let custom_impls = if this_enum.custom {
                quote! {
                    fn to_str(&self) -> &str {
                        match self {
                            Self::Null => "",
                            Self::Custom(s) => &s,  // Ensure match works for `Custom`
                            #(Self::#variants_strings_match_arms,)*
                        }
                    }
                    fn has_custom_variant() -> bool { true }
                    fn get_custom_value(&self) -> Option<&str> {
                        match self {
                            Self::Custom(value) => Some(value.as_str()),
                            _ => None,
                        }
                    }
                    fn try_set_custom(&mut self, new_value: &str) -> bool {
                        match self {
                            Self::Custom(value) => {
                                *value = new_value.to_string();
                                true
                            },
                            _ => false,
                        }
                    }
                    fn set_custom(&mut self, new_value: &str) -> bool {
                        *self = Self::Custom(new_value.to_string());
                        true
                    }
                    fn custom_variant(value: &str) -> Option<Self> {
                        Some(Self::Custom(value.to_string()))
                    }
                }
            } else {
                quote! {
                    fn to_str(&self) -> &str {
                        match self {
                            Self::Null => "",
                            #(Self::#variants_strings_match_arms,)*
                        }
                    }
                    fn has_custom_variant() -> bool { false }
                    fn get_custom_value(&self) -> Option<&str> { None }
                    fn try_set_custom(&mut self, new_value: &str) -> bool { false }
                    fn set_custom(&mut self, new_value: &str) -> bool { false }
                    fn custom_variant(value: &str) -> Option<Self> { None }
                }
            };

            let expanded = quote! {
                #[derive(EnumIter, Debug, PartialEq, Default, Clone)]
                pub enum #enum_name {
                    #[default]
                    #null_custom_variants
                    #(#variant_idents,)*
                }

                impl IntoEntryEnum for #enum_name {
                    fn name_str(&self) -> &str {
                        #enum_string_name
                    }
                    #custom_impls
                }
            };

            multiple_streams.push(expanded);
        });

    let combined_expanded = quote! {
        #(#multiple_streams)*
    };

    TokenStream::from(combined_expanded)
}
