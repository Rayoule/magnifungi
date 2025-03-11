use leptos::prelude::*;

use super::Entry;


#[derive(Clone)]
pub struct EntryValueUnit {
    pub value: f32,
    pub unit: String,
}
impl Entry for EntryValueUnit {}