use leptos::prelude::*;

use super::{Entry, FreeText};



#[derive(Clone)]
pub struct EntryTint {
    pub tint_desc: Option<FreeText>,
    pub color: Option<ColorPicker>,
}
impl Entry for EntryTint {}


#[derive(Clone)]
pub struct ColorPicker {
    pub color: String,
}
impl Entry for ColorPicker {}

