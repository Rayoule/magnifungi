use leptos::prelude::*;
use magnifungi_shared_types::entry_types::{entry_list::EntryList, entry_note::EntryNote, entry_trait::IntoFindView};





#[derive(Default, Clone)]
pub struct Notes {
    pub notes: EntryList<EntryNote>,
}
impl IntoFindView for Notes {}

