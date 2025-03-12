use super::{IntoFindView, FreeText};



#[derive(Clone)]
pub struct EntryTint {
    pub tint_desc: Option<FreeText>,
    pub color: Option<ColorPicker>,
}
impl IntoFindView for EntryTint {}


#[derive(Clone)]
pub struct ColorPicker {
    pub color: String,
}
impl IntoFindView for ColorPicker {}

