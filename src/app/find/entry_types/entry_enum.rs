use leptos::prelude::*;

use super::{Entry, FreeText};



pub struct EntryEnumCustomNote {
    selected_option: Option<usize>,
    options: Vec<String>,
    custom_option: String,
    note: FreeText,
}
impl EntryEnumerable for EntryEnumCustomNote {

    fn get_enum_options(&self) -> &[String] {
        &self.options[..]
    }

    fn get_custom_option(&self) -> Option<&str> {
        Some(self.custom_option.as_str())
    }

    fn get_note(&self) -> Option<&FreeText> {
        Some(&self.note)
    }
    
    fn has_custom_option(&self) -> bool {
        true
    }
    
    fn has_note(&self) -> bool {
        true
    }
}
impl Entry for EntryEnumCustomNote {}


pub struct EntryEnumNote {
    selected_option: Option<usize>,
    options: Vec<String>,
    note: FreeText,
}
impl EntryEnumerable for EntryEnumNote {

    fn get_enum_options(&self) -> &[String] {
        &self.options[..]
    }

    fn get_note(&self) -> Option<&FreeText> {
        Some(&self.note)
    }
    
    fn has_note(&self) -> bool {
        true
    }
}
impl Entry for EntryEnumNote {}


pub struct EntryEnumCustom {
    selected_option: Option<usize>,
    options: Vec<String>,
    custom_option: String,
}
impl EntryEnumerable for EntryEnumCustom {

    fn get_enum_options(&self) -> &[String] {
        &self.options[..]
    }

    fn get_custom_option(&self) -> Option<&str> {
        Some(self.custom_option.as_str())
    }
    
    fn has_custom_option(&self) -> bool {
        true
    }
}
impl Entry for EntryEnumCustom {}



pub struct EntryEnum {
    selected_option: Option<usize>,
    options: Vec<String>,
}
impl EntryEnumerable for EntryEnum {

    fn get_enum_options(&self) -> &[String] {
        &self.options[..]
    }
}
impl Entry for EntryEnum {}



pub trait EntryEnumerable {

    fn get_enum_options(&self) -> &[String];

    fn get_custom_option(&self) -> Option<&str> {
        None
    }

    fn get_note(&self) -> Option<&FreeText> {
        None
    }
    
    fn has_custom_option(&self) -> bool {
        false
    }
    
    fn has_note(&self) -> bool {
        false
    }
}

