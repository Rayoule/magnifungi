use leptos::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, Display};

use super::{entry_trait::Entry, FreeText};



pub mod entry_enum_list;

/// An EntryEnum that has:
/// - Custom option
/// - Note
#[derive(Clone, Default)]
pub struct EntryEnumCustomNote<T: IntoEntryEnumerator> {
    variant: T,
    custom: String,
    note: FreeText,
}
impl<T: IntoEntryEnumerator> Entry for EntryEnumCustomNote<T> {}
impl<T: IntoEntryEnumerator> HasEntryEnumerator<T> for EntryEnumCustomNote<T> {
    // Needed
    fn get_value(&self) -> &T { &self.variant }
    fn set_value(&mut self, value: T) { self.variant = value; }

    // Custom option
    fn has_custom_option() -> bool { true }
    fn get_custom(&self) -> Option<&str> { Some(self.custom.as_str()) }
    fn set_custom(&mut self, input: &str) { self.custom = input.to_string() }

    // Note
    fn has_note() -> bool { true }
    fn get_note(&self) -> Option<&FreeText> { Some(&self.note) }
    fn set_note(&mut self, input: &str) { self.note = FreeText {content: input.to_string()} }
}


/// An EntryEnum that also has
/// - Note
#[derive(Clone, Default)]
pub struct EntryEnumNote<T: IntoEntryEnumerator> {
    variant: T,
    note: FreeText,
}
impl<T: IntoEntryEnumerator> Entry for EntryEnumNote<T> {}
impl<T: IntoEntryEnumerator> HasEntryEnumerator<T> for EntryEnumNote<T> {
    // Needed
    fn get_value(&self) -> &T { &self.variant }
    fn set_value(&mut self, value: T) { self.variant = value; }

    // Note
    fn has_note() -> bool { true }
    fn get_note(&self) -> Option<&FreeText> { Some(&self.note) }
    fn set_note(&mut self, input: &str) { self.note = FreeText {content: input.to_string()} }
}


/// An EntryEnum that has
/// - Custom option
#[derive(Clone, Default)]
pub struct EntryEnumCustom<T: IntoEntryEnumerator> {
    variant: T,
    custom: String,
}
impl<T: IntoEntryEnumerator> Entry for EntryEnumCustom<T> {}
impl<T: IntoEntryEnumerator> HasEntryEnumerator<T> for EntryEnumCustom<T> {
    // Needed
    fn get_value(&self) -> &T { &self.variant }
    fn set_value(&mut self, value: T) { self.variant = value; }

    // Custom option
    fn has_custom_option() -> bool { true }
    fn get_custom(&self) -> Option<&str> { Some(self.custom.as_str()) }
    fn set_custom(&mut self, input: &str) { self.custom = input.to_string() }
}


/// An EntryEnum
#[derive(Clone, Default)]
pub struct EntryEnum<T: IntoEntryEnumerator> {
    variant: T,
}
impl<T: IntoEntryEnumerator> Entry for EntryEnum<T> {}
impl<T: IntoEntryEnumerator> HasEntryEnumerator<T> for EntryEnum<T> {
    fn get_value(&self) -> &T { &self.variant }
    fn set_value(&mut self, value: T) { self.variant = value; }
}



/// Describes an enum for a Find entry field
pub trait HasEntryEnumerator<T: IntoEntryEnumerator> {

    // Needed !
    fn get_value(&self) -> &T;
    fn set_value(&mut self, value: T);

    // Must fill if custom option !
    fn has_custom_option() -> bool { false }
    fn get_custom(&self) -> Option<&str> { None }
    fn set_custom(&mut self, _input: &str) {}

    // Must fill if has note !
    fn has_note() -> bool { false }
    fn get_note(&self) -> Option<&FreeText> { None }
    fn set_note(&mut self, _input: &str) {}

    // Don't override
    fn read_as_str<'a>(&'a self) -> &'a str where T: 'a {
        if self.get_value().is_custom() {
            if let Some(s) = self.get_custom() {
                s
            } else { self.get_value().to_str() }
        } else { self.get_value().to_str() }
    }
    fn get_all_variants(&self) -> Vec<String> {
        let iterator = T::iter();
        if Self::has_custom_option() {
            iterator
                .filter(|e: &T| !e.is_custom())
                .map(|e: T| e.to_str().to_string())
                .collect::<Vec<String>>()
        } else {
            iterator
                .map(|e: T| e.to_str().to_string())
                .collect::<Vec<String>>()
        }
    }
}


/// Describes an enum that can be used as an EntryEnumerator
/// Every Enums used for entries must implement this !
pub trait IntoEntryEnumerator: IntoEnumIterator {
    fn to_str(&self) -> &str;
    fn is_custom(&self) -> bool { false }
}
