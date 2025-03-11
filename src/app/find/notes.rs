use leptos::prelude::*;
use magnifungi_shared_types::entry_types::{entry_list::EntryList, entry_note::EntryNote};





#[derive(Default, Clone)]
pub struct Notes {
    pub notes: EntryList<EntryNote>,
}

