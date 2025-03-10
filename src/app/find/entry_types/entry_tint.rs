use leptos::prelude::*;

use super::{Entry, FreeText};



pub struct EntryTint {
    pub tint_desc: Option<FreeText>,
    pub color: Option<ColorPicker>,
}
impl Entry for EntryTint {}


pub struct ColorPicker {
    pub color: String,
}
impl Entry for ColorPicker {}

