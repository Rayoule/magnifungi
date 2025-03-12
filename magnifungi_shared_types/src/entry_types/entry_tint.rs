use::leptos::prelude::*;
use super::{Entry, FreeText, IntoFindView};



#[derive(Clone)]
pub struct EntryTint {
    pub tint_desc: Option<Entry<FreeText>>,
    pub color: Option<Entry<ColorPicker>>,
}
impl IntoFindView for EntryTint {
    fn into_any_view(&self) -> AnyView {
        let mut vec: Vec<AnyView> = Vec::new();
        if let Some(e) = &self.tint_desc {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.color {
            vec.push(e.into_any_view())
        }
        vec.into_any()
    }
}


#[derive(Clone)]
pub struct ColorPicker {
    pub color: String,
}
impl IntoFindView for ColorPicker {
    fn into_any_view(&self) -> AnyView {
        view! { <p>{ self.color.clone() }</p> }.into_any()
    }
}

