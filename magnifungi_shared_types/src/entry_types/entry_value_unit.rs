use leptos::prelude::*;

use super::IntoFindView;


#[derive(Clone)]
pub struct EntryValueUnit {
    pub value: f32,
    pub unit: String,
}
impl IntoFindView for EntryValueUnit {
    fn into_any_view(&self) -> AnyView {
        view! {
            <span>{ self.value.clone() }</span>
            <span>{ self.unit.clone() }</span>
        }.into_any()
    }
}