use leptos::prelude::*;

use super::IntoFindView;


#[derive(Clone)]
pub struct EntryValueUnit {
    pub value: f32,
    pub unit: String,
}
impl IntoFindView for EntryValueUnit {}