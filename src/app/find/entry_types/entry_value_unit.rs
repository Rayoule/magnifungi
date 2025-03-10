use leptos::prelude::*;

use super::Entry;


pub struct EntryValueUnit {
    pub value: f32,
    pub unit: String,
}
impl Entry for EntryValueUnit {}