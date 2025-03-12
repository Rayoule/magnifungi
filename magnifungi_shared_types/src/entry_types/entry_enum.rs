use leptos::prelude::{IntoAny, *};
use magnifungi_macros::generate_enums_from_path;
use strum::IntoEnumIterator;

use super::{entry_trait::IntoFindView, FreeText};


// Generate all IntoEntryEnum enums from enums.json
use strum_macros::EnumIter;
generate_enums_from_path!();



/// Describes an enum that can be used as an EntryEnumerator
/// Every Enums used for entries must implement this !
pub trait IntoEntryEnum: Send + Sync + IntoEnumIterator {
    fn to_str(&self) -> &str;
    fn name_str(&self) -> &str;
    fn has_custom_variant() -> bool;
    fn get_custom_value(&self) -> Option<&str>;
    fn try_set_custom(&mut self, new_value: &str) -> bool;
    fn set_custom(&mut self, new_value: &str) -> bool;
    fn custom_variant(value: &str) -> Option<Self>;
}


/// EntryEnum struct
/// - Note
#[derive(Clone, Default)]
pub struct EntryEnum<T: IntoEntryEnum> {
    pub variant: T,
    pub note: Option<FreeText>,
}
impl<T: IntoEntryEnum> EntryEnum<T> {
    // Custom option
    fn get_custom(&self) -> Option<&str> { self.variant.get_custom_value() }
}
impl<T: IntoEntryEnum> IntoFindView for EntryEnum<T> {
    fn into_any_view(&self) -> AnyView {
        todo!()
    }

    /*fn into_any_view(&self) -> AnyView {
        view! {
            <div>

                // Name
                <EntryEnumNameDisplay
                    enum_name=self.variant.name_str().to_string()
                />

                // Value
                <EntryEnumValueDisplay
                    variant_value=self.variant.name_str().to_string()
                    custom_value=self.get_custom().map(|s| s.to_string())
                />

                // Note
                {move || {
                    if let Some(note) = self.note {
                        view! {
                            <EntryEnumNoteDisplay
                                enum_note=note
                            />
                        }.into_any()
                    } else { ().into_any() }
                }}        
            </div>
        }.into_any()
    }*/
}




// Display Components

/// Display Value
#[component]
pub fn EntryEnumValueDisplay(variant_value: String) -> impl IntoView {
    view! {
        <p> { variant_value }</p>
    }
}

/// Display Name
#[component]
pub fn EntryEnumNameDisplay(enum_name: String) -> impl IntoView {
    view! {
        <p>{ enum_name.clone() }</p>
    }
}

/// Display Name
#[component]
pub fn EntryEnumNoteDisplay(enum_note: FreeText) -> impl IntoView {
    view! {
        {move || enum_note.into_any_view() }
    }
}
